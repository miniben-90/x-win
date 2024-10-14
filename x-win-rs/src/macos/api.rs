#![deny(unused_imports)]

use std::process::Command;

use base64::Engine;
use cocoa::appkit::NSScreen;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSRect, NSString, NSURL};
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
use core_graphics::geometry::CGRect;
use core_graphics::window::{
  kCGWindowBounds, kCGWindowIsOnscreen, kCGWindowLayer, kCGWindowMemoryUsage, kCGWindowName,
  kCGWindowNumber, kCGWindowOwnerName, kCGWindowOwnerPID,
};

use crate::common::x_win_struct::icon_info::IconInfo;
use crate::common::{
  api::{empty_entity, os_name, Api},
  x_win_struct::{
    process_info::ProcessInfo, usage_info::UsageInfo, window_info::WindowInfo,
    window_position::WindowPosition,
  },
};

use objc::runtime::{BOOL, NO};

pub struct MacosAPI {}

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
    if window_info.info.path.ne("") {
      unsafe {
        let path = NSString::alloc(nil).init_str(&window_info.info.path);
        let nsworkspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let nsimage: id = msg_send![nsworkspace, iconForFile: path];

        if !nsimage.is_null() {
          let cgref: id = msg_send![
            nsimage,
            CGImageForProposedRect: nil
            context: nil
            hints: nil
          ];
          let nsbitmapref: id = msg_send![class!(NSBitmapImageRep), alloc];
          let imagerep: id = msg_send![nsbitmapref, initWithCGImage: cgref];
          let imagesize: (f64, f64) = msg_send![nsimage, size];
          let _: () = msg_send![imagerep, setSize: imagesize];
          let pngdata: id = msg_send![imagerep, representationUsingType:4 properties:nil];
          let length: usize = msg_send![pngdata, length];
          let bytes: *const u8 = msg_send![pngdata, bytes];
          let byte_slice: &[u8] = std::slice::from_raw_parts(bytes, length);
          let data = base64::prelude::BASE64_STANDARD.encode(byte_slice);
          return IconInfo {
            data: format!("data:image/png;base64,{}", data),
            width: imagesize.0 as u32,
            height: imagesize.1 as u32,
          };
        }
      }
    }
    IconInfo {
      data: "".to_owned(),
      height: 0,
      width: 0,
    }
  }

  fn get_browser_url(&self, window_info: &WindowInfo) -> String {
    get_browser_url(window_info.id, window_info.info.process_id)
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
    let bounds = CGRect::from_dict_representation(&bounds.to_untyped());
    if bounds.is_none() {
      continue;
    }

    let bounds: CGRect = bounds.unwrap();
    if bounds.size.height.lt(&50.0) || bounds.size.width.lt(&50.0) {
      continue;
    }

    let process_id = cfd.get(unsafe { kCGWindowOwnerPID });
    let process_id = process_id.downcast::<CFNumber>().unwrap().to_i64().unwrap();

    let app: id = unsafe {
      msg_send![
        class!(NSRunningApplication),
        runningApplicationWithProcessIdentifier: process_id
      ]
    };

    let is_not_active: bool = unsafe {
      let is_active: BOOL = msg_send![app, isActive];
      is_active == NO
    };

    if only_active && is_not_active {
      continue;
    }

    let bundle_identifier: id = unsafe { msg_send![app, bundleIdentifier] };
    let bundle_identifier = unsafe { NSString::UTF8String(bundle_identifier) };
    let bundle_identifier =
      std::str::from_utf8(unsafe { std::ffi::CStr::from_ptr(bundle_identifier).to_bytes() })
        .unwrap();

    if bundle_identifier.eq("com.apple.dock") {
      continue;
    }

    let app_name = cfd.get(unsafe { kCGWindowOwnerName });
    let app_name = app_name.downcast::<CFString>().unwrap().to_string();

    let mut title: String = "".to_owned();

    if cfd.contains_key(&CFString::from_static_string("kCGWindowName")) {
      let title_ref = cfd.get(unsafe { kCGWindowName });
      title = title_ref.downcast::<CFString>().unwrap().to_string();
    }

    let bundle_url: id = unsafe { msg_send![app, bundleURL] };
    let path = unsafe { bundle_url.path().UTF8String() };
    let path = std::str::from_utf8(unsafe { std::ffi::CStr::from_ptr(path).to_bytes() }).unwrap();
    let exec_name = std::path::Path::new(&app_name)
      .file_name()
      .unwrap()
      .to_str()
      .unwrap();

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
    return String::from_utf8_lossy(&output.stdout).trim().to_owned();
  }
  "".into()
}

fn get_screen_rect() -> NSRect {
  let screen = unsafe { NSScreen::mainScreen(nil) };
  unsafe { NSScreen::frame(screen) }
}

fn is_full_screen(window_rect: CGRect, screen_rect: NSRect) -> bool {
  window_rect.size.height.eq(&screen_rect.size.height)
    && window_rect.size.width.eq(&screen_rect.size.width)
    && window_rect.origin.y.eq(&screen_rect.origin.y)
    && window_rect.origin.x.eq(&screen_rect.origin.x)
}

fn get_browser_url(window_id: u32, process_id: u32) -> String {
  let mut url: String = String::new();

  if is_browser_bundle_id(bundle_identifier) {
    let mut command = format!(
      "tell app id \"{}\" to get URL of active tab of front window",
      bundle_identifier
    );
    if is_from_document(bundle_identifier) {
      command = format!(
        "tell app id \"{}\" to get URL of front document",
        bundle_identifier
      );
    }
    // else if is_firefox_browser(&bundle_identifier)
    // {
    //   command = format!("tell app id \"{}\" to get URL of active tab of front window", bundle_identifier);
    // }
    url = execute_applescript(&command);
  }
}
