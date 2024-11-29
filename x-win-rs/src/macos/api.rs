#![deny(unused_imports)]

use std::process::Command;
use std::ptr::null_mut;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use libc::pid_t;
use objc2::rc::Retained;
use objc2::{ClassType, Encode, RefEncode};
use objc2_app_kit::{
  NSBitmapImageFileType, NSBitmapImageRep, NSImage, NSRunningApplication, NSScreen, NSWorkspace,
};
use objc2_foundation::{
  CGPoint, CGRect, CGSize, MainThreadMarker, NSDictionary, NSObject, NSRect, NSString,
};

use core_foundation::array::{CFArrayGetCount, CFArrayGetValueAtIndex};
use core_foundation::base::{CFType, TCFType};
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;
use core_graphics::display::{
  kCGWindowListExcludeDesktopElements, kCGWindowListOptionIncludingWindow,
  kCGWindowListOptionOnScreenOnly, CGWindowListCopyWindowInfo,
};
use core_graphics::geometry::CGRect as GeCGRect;
use core_graphics::window::{
  kCGWindowBounds, kCGWindowIsOnscreen, kCGWindowLayer, kCGWindowMemoryUsage, kCGWindowName,
  kCGWindowNumber, kCGWindowOwnerName, kCGWindowOwnerPID,
};

use crate::common::x_win_struct::icon_info::IconInfo;
use crate::common::{
  api::{empty_entity, empty_icon, os_name, Api},
  x_win_struct::{
    process_info::ProcessInfo, usage_info::UsageInfo, window_info::WindowInfo,
    window_position::WindowPosition,
  },
};

pub struct MacosAPI {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CGImage {}

unsafe impl Encode for CGImage {
  const ENCODING: objc2::Encoding = objc2::Encoding::Struct("CGImage", &[]);
}

unsafe impl RefEncode for CGImage {
  const ENCODING_REF: objc2::Encoding = objc2::Encoding::Pointer(&Self::ENCODING);
}

/**
 * Impl. for Darwin system
 */
impl Api for MacosAPI {
  fn get_active_window(&self) -> WindowInfo {
    let windows: Vec<WindowInfo> = get_windows_informations(true);
    if !windows.is_empty() {
      let t: &WindowInfo = windows.first().unwrap();
      t.clone() as WindowInfo
    } else {
      empty_entity()
    }
  }

  fn get_open_windows(&self) -> Vec<WindowInfo> {
    get_windows_informations(false)
  }

  fn get_app_icon(&self, window_info: &WindowInfo) -> IconInfo {
    if !window_info.info.path.is_empty() {
      let path: &NSString = &NSString::from_str(&window_info.info.path);

      let nsimage: &NSImage = unsafe { &NSWorkspace::sharedWorkspace().iconForFile(path) };
      if unsafe { nsimage.isValid() } {
        let imagesize = unsafe { nsimage.size() };
        let rect: &CGRect = &CGRect::new(CGPoint::new(0.0, 0.0), CGSize::new(0.0, 0.0));
        let cgref: &CGImage = unsafe {
          msg_send![nsimage, CGImageForProposedRect: rect context: null_mut::<NSObject>() hints: null_mut::<NSObject>()]
        };

        let nsbitmapref = NSBitmapImageRep::alloc();
        let imagerep: Retained<NSBitmapImageRep> =
          unsafe { msg_send_id![nsbitmapref, initWithCGImage: cgref] };
        let _: () = unsafe { imagerep.setSize(imagesize) };
        let pngdata = unsafe {
          imagerep
            .representationUsingType_properties(NSBitmapImageFileType::PNG, &NSDictionary::new())
        };
        match pngdata {
          Some(pngdata) => {
            let bytes = pngdata.bytes();
            let data = BASE64_STANDARD.encode(bytes);
            let t = IconInfo {
              data: format!("data:image/png;base64,{}", data),
              width: imagesize.width as u32,
              height: imagesize.height as u32,
            };
            return t;
          }
          None => {
            return empty_icon();
          }
        }
      }
    }
    empty_icon()
  }

  fn get_browser_url(&self, window_info: &WindowInfo) -> String {
    get_browser_url(window_info.info.process_id)
  }
}

fn get_windows_informations(only_active: bool) -> Vec<WindowInfo> {
  let mut windows: Vec<WindowInfo> = Vec::new();

  let options = kCGWindowListOptionOnScreenOnly
    | kCGWindowListExcludeDesktopElements
    | kCGWindowListOptionIncludingWindow;
  let window_list_info = unsafe { CGWindowListCopyWindowInfo(options, 0) };
  let windows_count: isize = unsafe { CFArrayGetCount(window_list_info) };

  let screen_rect = get_screen_rect();

  for idx in 0..windows_count {
    let dref: CFDictionaryRef =
      unsafe { CFArrayGetValueAtIndex(window_list_info, idx) as CFDictionaryRef };

    if dref.is_null() {
      continue;
    }

    let cfd: CFDictionary<CFString, CFType> = unsafe { CFDictionary::wrap_under_create_rule(dref) };

    let is_screen = cfd.get(unsafe { kCGWindowIsOnscreen });
    let is_screen: CFBoolean = is_screen.downcast::<CFBoolean>().unwrap();
    if is_screen != CFBoolean::true_value() {
      continue;
    }

    let window_layer = cfd.get(unsafe { kCGWindowLayer });
    let window_layer: CFNumber = window_layer.downcast::<CFNumber>().unwrap();
    if window_layer.lt(&CFNumber::from(0)) || window_layer.gt(&CFNumber::from(100)) {
      continue;
    }

    let bounds = cfd.get(unsafe { kCGWindowBounds });
    let bounds: CFDictionary = bounds.downcast::<CFDictionary>().unwrap();
    let bounds = GeCGRect::from_dict_representation(&bounds.to_untyped());
    if bounds.is_none() {
      continue;
    }

    let bounds: GeCGRect = bounds.unwrap();
    if bounds.size.height.lt(&50.0) || bounds.size.width.lt(&50.0) {
      continue;
    }
    let process_id = cfd.get(unsafe { kCGWindowOwnerPID });
    let process_id = process_id.downcast::<CFNumber>().unwrap().to_i64().unwrap();
    let app: &NSRunningApplication = unsafe {
      msg_send![
        class!(NSRunningApplication),
        runningApplicationWithProcessIdentifier: pid_t::from(process_id as i32)
      ]
    };

    let is_not_active = !unsafe { app.isActive() };

    if only_active && is_not_active {
      continue;
    }

    let bundle_identifier = get_bundle_identifier(app);

    if bundle_identifier.eq("com.apple.dock") {
      continue;
    }

    let app_name = {
      let app_name = cfd.get(unsafe { kCGWindowOwnerName });
      app_name
        .downcast::<CFString>()
        .unwrap_or("".into())
        .to_string()
    };

    let mut title: String = String::from("");

    if cfd.contains_key(&CFString::from_static_string("kCGWindowName")) {
      let title_ref = cfd.get(unsafe { kCGWindowName });
      title = title_ref.downcast::<CFString>().unwrap().to_string();
    }

    let path: String = unsafe {
      match app.bundleURL() {
        Some(nsurl) => match nsurl.path() {
          Some(path) => path.to_string(),
          None => String::from(""),
        },
        None => String::from(""),
      }
    };

    let exec_name: String = {
      match path.is_empty() {
        true => match std::path::Path::new(&path).file_name() {
          Some(os_str) => match os_str.to_str() {
            Some(exec_name) => exec_name.to_owned(),
            None => String::from(""),
          },
          None => String::from(""),
        },
        false => String::from(""),
      }
    };

    let memory = cfd.get(unsafe { kCGWindowMemoryUsage });
    let memory = memory.downcast::<CFNumber>().unwrap().to_i64().unwrap();

    let id = cfd.get(unsafe { kCGWindowNumber });
    let id = id.downcast::<CFNumber>().unwrap().to_i64().unwrap();

    windows.push(WindowInfo {
      id: id as u32,
      os: os_name(),
      title,
      position: WindowPosition {
        x: bounds.origin.x as i32,
        y: bounds.origin.y as i32,
        width: bounds.size.width as i32,
        height: bounds.size.height as i32,
        is_full_screen: is_full_screen(bounds, screen_rect),
      },
      info: ProcessInfo {
        process_id: process_id as u32,
        path: path.to_owned(),
        name: app_name.to_owned(),
        exec_name: exec_name.to_owned(),
      },
      usage: UsageInfo {
        memory: memory as u32,
      },
    });

    if only_active && is_not_active {
      break;
    }
  }

  windows
}

fn is_browser_bundle_id(bundle_id: &str) -> bool {
  matches!(
    bundle_id,
    "com.apple.Safari"
      | "com.apple.SafariTechnologyPreview"
      | "com.google.Chrome"
      | "com.google.Chrome.beta"
      | "com.google.Chrome.dev"
      | "com.google.Chrome.canary"
      | "org.mozilla.firefox"
      | "org.mozilla.firefoxdeveloperedition"
      | "com.brave.Browser"
      | "com.brave.Browser.beta"
      | "com.brave.Browser.nightly"
      | "com.microsoft.edgemac"
      | "com.microsoft.edgemac.Beta"
      | "com.microsoft.edgemac.Dev"
      | "com.microsoft.edgemac.Canary"
      | "com.mighty.app"
      | "com.ghostbrowser.gb1"
      | "com.bookry.wavebox"
      | "com.pushplaylabs.sidekick"
      | "com.operasoftware.Opera"
      | "com.operasoftware.OperaNext"
      | "com.operasoftware.OperaDeveloper"
      | "com.operasoftware.OperaGX"
      | "com.vivaldi.Vivaldi"
      | "com.kagi.kagimacOS"
      | "company.thebrowser.Browser"
      | "com.sigmaos.sigmaos.macos"
      | "com.SigmaOS.SigmaOS"
  )
}

fn is_from_document(bundle_id: &str) -> bool {
  matches!(
    bundle_id,
    "com.apple.Safari" | "com.apple.SafariTechnologyPreview" | "com.kagi.kagimacOS"
  )
}

// fn is_firefox_browser(bundle_id: &str) -> bool {
//   match bundle_id {
//     | "org.mozilla.firefox"
//     | "org.mozilla.firefoxdeveloperedition" => true,
//     _ => false,
//   }
// }

fn execute_applescript(script: &str) -> String {
  let output = Command::new("osascript").args(["-e", script]).output();
  if let Ok(output) = output {
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
  } else {
    "".into()
  }
}

fn get_screen_rect() -> NSRect {
  if let Some(screen) = unsafe { NSScreen::mainScreen(MainThreadMarker::new_unchecked()) } {
    screen.frame()
  } else {
    NSRect::new(CGPoint::new(0.0, 0.0), CGSize::new(0.0, 0.0))
  }
}

fn is_full_screen(window_rect: GeCGRect, screen_rect: NSRect) -> bool {
  window_rect.size.height.eq(&screen_rect.size.height)
    && window_rect.size.width.eq(&screen_rect.size.width)
    && window_rect.origin.y.eq(&screen_rect.origin.y)
    && window_rect.origin.x.eq(&screen_rect.origin.x)
}

fn get_browser_url(process_id: u32) -> String {
  let process_id = process_id as i64;
  let app: &NSRunningApplication = unsafe {
    msg_send![
      class!(NSRunningApplication),
      runningApplicationWithProcessIdentifier: pid_t::from(process_id as i32)
    ]
  };

  let bundle_identifier = get_bundle_identifier(app);
  if bundle_identifier.is_empty() {
    return String::from("");
  }

  if is_browser_bundle_id(&bundle_identifier) {
    let mut command = format!(
      "tell app id \"{}\" to get URL of active tab of front window",
      bundle_identifier
    );
    if is_from_document(&bundle_identifier) {
      command = format!(
        "tell app id \"{}\" to get URL of front document",
        bundle_identifier
      );
    }
    // else if is_firefox_browser(&bundle_identifier)
    // {
    //   command = format!("tell app id \"{}\" to get URL of active tab of front window", bundle_identifier);
    // }
    execute_applescript(&command)
  } else {
    String::from("")
  }
}

fn get_bundle_identifier(app: &NSRunningApplication) -> String {
  unsafe {
    match app.bundleIdentifier() {
      Some(bundle_identifier) => bundle_identifier.to_string(),
      None => String::from(""),
    }
  }
}
