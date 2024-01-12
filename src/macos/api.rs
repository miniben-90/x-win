#![deny(unused_imports)]

use std::process::Command;

use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString, NSUInteger, NSURL};
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

use crate::common::{
  api::API,
  x_win_struct::{
    process_info::ProcessInfo, usage_info::UsageInfo, window_info::WindowInfo,
    window_position::WindowPosition,
  },
};

pub struct MacosAPI {}

/**
 * Impl. for windows system
 */
impl API for MacosAPI {
  fn get_active_window(&self) -> WindowInfo {
    let windows: Vec<WindowInfo> = get_windows_informations(true);
    if windows.len() > 0 {
      let t: &WindowInfo = windows.first().unwrap();
      t.clone() as WindowInfo
    } else {
      WindowInfo {
        id: 0,
        os: os_name(),
        title: "".to_owned(),
        position: WindowPosition {
          x: 0,
          y: 0,
          width: 0,
          height: 0,
        },
        info: ProcessInfo {
          process_id: 0,
          path: "".to_owned(),
          name: "".to_owned(),
          exec_name: "".to_owned(),
        },
        usage: UsageInfo { memory: 0 },
        url: "".to_owned(),
      }
    }
  }

  fn get_open_windows(&self) -> Vec<WindowInfo> {
    get_windows_informations(false)
  }
}

/**
 * To know the os
 */
fn os_name() -> String {
  r#"darwin"#.to_owned()
}

// fn get_window_information

fn get_active_window_pid() -> NSUInteger {
  unsafe {
    let _pool = NSAutoreleasePool::new(nil);
    let _shared_app_id: id = msg_send![class!(NSApplication), sharedApplication];
    // NSApplication::finishLaunching(shared_app_id);
    let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
    let frontapp: id = msg_send![workspace, frontmostApplication];
    let active_window_pid: NSUInteger = msg_send![frontapp, processIdentifier];
    return active_window_pid;
  }
}

fn get_windows_informations(only_active: bool) -> Vec<WindowInfo> {
  let mut windows: Vec<WindowInfo> = Vec::new();
  let mut active_window_pid: u64 = 0;

  if only_active {
    active_window_pid = get_active_window_pid();
  }

  let options = kCGWindowListOptionOnScreenOnly
    | kCGWindowListExcludeDesktopElements
    | kCGWindowListOptionIncludingWindow;
  let window_list_info = unsafe { CGWindowListCopyWindowInfo(options, 0) };
  let windows_count: isize = unsafe { CFArrayGetCount(window_list_info) };
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

    if only_active && process_id.ne(&(active_window_pid as i64)) {
      continue;
    }

    let app: id = unsafe {
      msg_send![
        class!(NSRunningApplication),
        runningApplicationWithProcessIdentifier: process_id
      ]
    };

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

    let mut url: String = String::new();

    if is_browser_bundle_id(&bundle_identifier) {
      let mut command = format!("tell app id \"{}\" to get URL of active tab of front window", bundle_identifier);
      if is_from_document(&bundle_identifier)
      {
        command = format!("tell app id \"{}\" to get URL of front document", bundle_identifier);
      }
      // else if is_firefox_browser(&bundle_identifier)
      // {
      //   command = format!("tell app id \"{}\" to get URL of active tab of front window", bundle_identifier);
      // }
      url = execute_applescript(&command);
    }


    windows.push(WindowInfo {
      id: id as u32,
      os: os_name(),
      title,
      position: WindowPosition {
        x: bounds.origin.x as i32,
        y: bounds.origin.y as i32,
        width: bounds.size.width as i32,
        height: bounds.size.height as i32,
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
      url,
    });

    if only_active && process_id.ne(&(active_window_pid as i64)) {
      break;
    }
  }

  return windows;
}

fn is_browser_bundle_id(bundle_id: &str) -> bool {
  match bundle_id {
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
    | "com.SigmaOS.SigmaOS" => true,
    _ => false,
  }
}

fn is_from_document(bundle_id: &str) -> bool {
  match bundle_id {
    "com.apple.Safari"
    | "com.apple.SafariTechnologyPreview"
    | "com.kagi.kagimacOS" => true,
    _ => false,
  }
}

fn is_firefox_browser(bundle_id: &str) -> bool {
  match bundle_id {
    | "org.mozilla.firefox"
    | "org.mozilla.firefoxdeveloperedition" => true,
    _ => false,
  }
}

fn execute_applescript(script: &str) -> String {
  let output = Command::new("osascript")
  .args(&["-e", script])
  .output();
  if output.is_ok() {
    return String::from_utf8_lossy(&output.unwrap().stdout).trim().to_owned();
  }
  return "".to_owned();
}
