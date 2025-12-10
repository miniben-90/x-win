#![deny(unused_imports)]

use std::ffi::c_void;
use std::process::Command;
use std::ptr::null_mut;

use crate::common::x_win_struct::icon_info::IconInfo;
use crate::common::{
  api::{empty_entity, empty_icon, os_name, Api},
  result::Result,
  x_win_struct::{
    process_info::ProcessInfo, usage_info::UsageInfo, window_info::WindowInfo,
    window_position::WindowPosition,
  },
};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use objc2::rc::{autoreleasepool, Retained};
use objc2::{AllocAnyThread, Encode, RefEncode};
use objc2_app_kit::{
  NSBitmapImageFileType, NSBitmapImageRep, NSImage, NSRunningApplication, NSScreen, NSWorkspace,
};
use objc2_core_foundation::{
  CFArray, CFBoolean, CFDictionary, CFNumber, CFNumberType, CFRetained, CFString, CGPoint, CGRect,
  CGSize,
};
use objc2_core_graphics::{
  kCGNullWindowID, CGRectMakeWithDictionaryRepresentation, CGWindowListCopyWindowInfo,
  CGWindowListOption,
};
use objc2_foundation::{MainThreadMarker, NSDictionary, NSObject, NSRect, NSString};

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
  fn get_active_window(&self) -> Result<WindowInfo> {
    let windows: Vec<WindowInfo> = get_windows_informations(true)?;
    let active_window = {
      if !windows.is_empty() {
        match windows.first() {
          Some(active_window) => active_window.clone(),
          None => empty_entity(),
        }
      } else {
        empty_entity()
      }
    };
    Ok(active_window)
  }

  fn get_open_windows(&self) -> Result<Vec<WindowInfo>> {
    get_windows_informations(false)
  }

  fn get_app_icon(&self, window_info: &WindowInfo) -> Result<IconInfo> {
    autoreleasepool(|_pool| get_app_icon(window_info))
  }

  fn get_browser_url(&self, window_info: &WindowInfo) -> Result<String> {
    autoreleasepool(|_pool| get_browser_url(window_info.info.process_id))
  }
}

fn get_app_icon(window_info: &WindowInfo) -> Result<IconInfo> {
  if !window_info.info.path.is_empty() {
    let path: &NSString = &NSString::from_str(&window_info.info.path);

    let nsimage: &NSImage = &NSWorkspace::sharedWorkspace().iconForFile(path);
    if nsimage.isValid() {
      let imagesize = nsimage.size();
      let rect: &CGRect = &CGRect::new(CGPoint::new(0.0, 0.0), CGSize::new(0.0, 0.0));
      let cgref: &CGImage = unsafe {
        msg_send![nsimage, CGImageForProposedRect: rect, context: null_mut::<NSObject>(), hints: null_mut::<NSObject>()]
      };
      let nsbitmapref = NSBitmapImageRep::alloc();
      let imagerep: Retained<NSBitmapImageRep> =
        unsafe { msg_send![nsbitmapref, initWithCGImage: cgref] };
      let _: () = imagerep.setSize(imagesize);
      let pngdata = unsafe {
        imagerep
          .representationUsingType_properties(NSBitmapImageFileType::PNG, &NSDictionary::new())
      };
      match pngdata {
        Some(pngdata) => {
          let bytes = unsafe { pngdata.as_bytes_unchecked() };
          let data = BASE64_STANDARD.encode(bytes);
          let icon = IconInfo {
            data: format!("data:image/png;base64,{data}"),
            width: imagesize.width as u32,
            height: imagesize.height as u32,
          };
          return Ok(icon);
        }
        None => {
          return Ok(empty_icon());
        }
      }
    }
  }
  Ok(empty_icon())
}

fn get_windows_informations(only_active: bool) -> Result<Vec<WindowInfo>> {
  autoreleasepool(|_pool| get_windows_informations_inner(only_active))
}

fn get_windows_informations_inner(only_active: bool) -> Result<Vec<WindowInfo>> {
  let mut windows: Vec<WindowInfo> = Vec::new();

  let option = CGWindowListOption::OptionOnScreenOnly
    | CGWindowListOption::ExcludeDesktopElements
    | CGWindowListOption::OptionIncludingWindow;
  let window_list_info: &CFArray = &CGWindowListCopyWindowInfo(option, kCGNullWindowID).unwrap();
  let windows_count = CFArray::count(window_list_info);
  let screen_rect = get_screen_rect();

  for idx in 0..windows_count {
    let window_cf_dictionary_ref =
      unsafe { CFArray::value_at_index(window_list_info, idx) as *const CFDictionary };

    if window_cf_dictionary_ref.is_null() {
      continue;
    }
    let window_cf_dictionary =
      unsafe { CFRetained::retain(std::ptr::NonNull::from(&*window_cf_dictionary_ref)) };
    let is_screen: bool = get_cf_boolean_value(&window_cf_dictionary, "kCGWindowIsOnscreen");
    if !is_screen {
      continue;
    }

    let window_layer = get_cf_number_value(&window_cf_dictionary, "kCGWindowLayer");

    if window_layer.lt(&0) || window_layer.gt(&100) {
      continue;
    }

    let bounds = get_cf_window_bounds_value(&window_cf_dictionary);

    if bounds.is_none() {
      continue;
    }

    let bounds = match bounds {
      Some(bounds) => bounds,
      None => CGRect {
        origin: CGPoint { x: 0.0, y: 0.0 },
        size: CGSize {
          width: 0.0,
          height: 0.0,
        },
      },
    };

    if bounds.size.height.lt(&50.0) || bounds.size.width.lt(&50.0) {
      continue;
    }

    let process_id = get_cf_number_value(&window_cf_dictionary, "kCGWindowOwnerPID");
    if process_id == 0 {
      continue;
    }

    let app = get_running_application_from_pid(process_id as u32);
    if app.is_err() {
      continue;
    }
    let app = app.unwrap();

    let is_not_active = !app.isActive();

    if only_active && is_not_active {
      continue;
    }

    let bundle_identifier = get_bundle_identifier(app);

    if bundle_identifier.eq("com.apple.dock") {
      continue;
    }

    let app_name = get_cf_string_value(&window_cf_dictionary, "kCGWindowOwnerName");
    let title = get_cf_string_value(&window_cf_dictionary, "kCGWindowName");

    let (path, exec_name) = {
      let mut path: String = String::new();
      let mut exec_name: String = String::new();
      match app.bundleURL() {
        Some(nsurl) => {
          if let Some(nsurl) = nsurl.path() {
            path = nsurl.to_string();
            exec_name = std::path::Path::new(&path)
              .file_name()
              .unwrap_or_default()
              .to_str()
              .unwrap_or_default()
              .to_string();
          }
        }
        None => {
          path = String::from("");
          exec_name = String::from("");
        }
      }
      (path, exec_name)
    };

    let memory = get_cf_number_value(&window_cf_dictionary, "kCGWindowMemoryUsage");
    let id = get_cf_number_value(&window_cf_dictionary, "kCGWindowNumber");
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

  Ok(windows)
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
    String::from("")
  }
}

fn get_screen_rect() -> NSRect {
  if let Some(screen) = unsafe { NSScreen::mainScreen(MainThreadMarker::new_unchecked()) } {
    screen.frame()
  } else {
    NSRect::new(CGPoint::new(0.0, 0.0), CGSize::new(0.0, 0.0))
  }
}

fn is_full_screen(window_rect: CGRect, screen_rect: NSRect) -> bool {
  window_rect.size.height.eq(&screen_rect.size.height)
    && window_rect.size.width.eq(&screen_rect.size.width)
    && window_rect.origin.y.eq(&screen_rect.origin.y)
    && window_rect.origin.x.eq(&screen_rect.origin.x)
}

// Recover browser url using process id to get it
fn get_browser_url(process_id: u32) -> Result<String> {
  let app = get_running_application_from_pid(process_id);

  match app {
    Ok(app) => {
      let bundle_identifier = get_bundle_identifier(app);
      if bundle_identifier.is_empty() || !is_browser_bundle_id(&bundle_identifier) {
        return Ok(String::from(""));
      }
      let mut command =
        format!("tell app id \"{bundle_identifier}\" to get URL of active tab of front window");
      if is_from_document(&bundle_identifier) {
        command = format!("tell app id \"{bundle_identifier}\" to get URL of front document");
      }
      // else if is_firefox_browser(&bundle_identifier)
      // {
      //   command = format!("tell app id \"{}\" to get URL of active tab of front window", bundle_identifier);
      // }
      Ok(execute_applescript(&command))
    }
    Err(_) => Ok(String::from("")),
  }
}

fn get_bundle_identifier(app: &NSRunningApplication) -> String {
  match app.bundleIdentifier() {
    Some(bundle_identifier) => bundle_identifier.to_string(),
    None => String::from(""),
  }
}

fn get_cf_dictionary_get_value<T>(dict: &CFDictionary, key: &str) -> Option<*const T> {
  let key = CFString::from_str(key);
  let key_ref = key.as_ref() as *const CFString;
  if unsafe { CFDictionary::contains_ptr_key(dict, key_ref.cast()) } {
    let value = unsafe { CFDictionary::value(dict, key_ref.cast()) };
    Some(value as *const T)
  } else {
    None
  }
}

// fn str_to_cfstring(key: &str) -> *const CFString {
//   let cf_dictionary_key = CFString::from_str(key);
// }

fn get_cf_number_value(dict: &CFDictionary, key: &str) -> i32 {
  unsafe {
    let mut value: i32 = 0;
    match get_cf_dictionary_get_value::<CFNumber>(dict, key) {
      Some(number) => {
        CFNumber::value(
          &*number,
          CFNumberType::IntType,
          &mut value as *mut _ as *mut c_void,
        );
        value
      }
      None => value,
    }
  }
}

fn get_cf_boolean_value(dict: &CFDictionary, key: &str) -> bool {
  unsafe {
    match get_cf_dictionary_get_value::<CFBoolean>(dict, key) {
      Some(value) => CFBoolean::value(&*value),
      None => false,
    }
  }
}

fn get_cf_window_bounds_value(dict: &CFDictionary) -> Option<CGRect> {
  match get_cf_dictionary_get_value::<CFDictionary>(dict, "kCGWindowBounds") {
    Some(dict_react) => unsafe {
      let mut cg_rect = CGRect::default();
      if !dict_react.is_null()
        && CGRectMakeWithDictionaryRepresentation(Some(&*dict_react), &mut cg_rect)
      {
        Some(cg_rect as CGRect)
      } else {
        None
      }
    },
    None => None,
  }
}

fn get_cf_string_value(dict: &CFDictionary, key: &str) -> String {
  unsafe {
    match get_cf_dictionary_get_value::<CFString>(dict, key) {
      Some(value) => (*value).to_string(),
      None => String::from(""),
    }
  }
}

fn get_running_application_from_pid(process_id: u32) -> Result<&'static NSRunningApplication> {
  let process_id = process_id as i64;
  let app: *mut NSRunningApplication = unsafe {
    msg_send![
      class!(NSRunningApplication),
      runningApplicationWithProcessIdentifier: process_id as i32
    ]
  };
  if app.is_null() {
    Err(String::from("Application not found with pid").into())
  } else {
    Ok(unsafe { &*app })
  }
}
