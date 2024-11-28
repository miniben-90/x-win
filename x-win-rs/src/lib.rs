#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc2;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate core;

mod common;

#[cfg(target_os = "windows")]
mod win32;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
use win32::init_platform_api;

#[cfg(target_os = "linux")]
use linux::init_platform_api;

#[cfg(target_os = "macos")]
use macos::init_platform_api;

#[cfg(all(feature = "macos_permission", target_os = "macos"))]
pub use macos::permission;

pub use common::{
  api::{empty_entity, os_name},
  x_win_struct::{
    icon_info::IconInfo, process_info::ProcessInfo, usage_info::UsageInfo, window_info::WindowInfo,
    window_position::WindowPosition,
  },
};

use crate::common::api::Api;

use std::fmt;

#[derive(Debug)]
pub struct XWinError;

impl fmt::Display for XWinError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Oops something got wrong with x-win")
  }
}

impl std::error::Error for XWinError {}

/**
 * Recover icon of window.
 * Return `IconInfo`
 */
pub fn get_window_icon(window_info: &WindowInfo) -> Result<IconInfo, XWinError> {
  let api = init_platform_api();
  Ok(api.get_app_icon(window_info))
}

/**
 * Recover browser url of window.
 * Return `String`
 */
pub fn get_browser_url(window_info: &WindowInfo) -> Result<String, XWinError> {
  let api = init_platform_api();
  Ok(api.get_browser_url(window_info))
}

/**
 * Retrieve information the about currently active window.
 * Return `WindowInfo` containing details about a specific active window.
 */
pub fn get_active_window() -> Result<WindowInfo, XWinError> {
  let api = init_platform_api();
  Ok(api.get_active_window())
}

/**
 * Retrieve information about the currently open windows.
 * Return `Vec<WindowInfo>` each containing details about a specific open window.
 */
pub fn get_open_windows() -> Result<Vec<WindowInfo>, XWinError> {
  let api = init_platform_api();
  Ok(api.get_open_windows())
}

/**
 * Install "@mininben90/x-win" Gnome extensions required for Linux using Gnome > 41.
 * This function will write extension files needed to correctly detect working windows with Wayland desktop environment.
 * **Restart session will be require to install the gnome extension.**
 */
pub fn install_extension() -> Result<bool, XWinError> {
  #[cfg(not(target_os = "linux"))]
  {
    Ok(false)
  }
  #[cfg(target_os = "linux")]
  {
    Ok(linux::gnome_install_extension())
  }
}

/**
 * Uninstall "@mininben90/x-win" Gnome extensions.
 * This function will disable and remove extension files.
 * **Restart session will be require to remove the gnome extension.**
 */
pub fn uninstall_extension() -> Result<bool, XWinError> {
  #[cfg(not(target_os = "linux"))]
  {
    Ok(false)
  }
  #[cfg(target_os = "linux")]
  {
    Ok(linux::gnome_uninstall_extension())
  }
}

/**
 * Enable Gnome extensions required for Linux using Gnome > 41.
 * This function will enable extension needed to correctly detect working windows with Wayland desktop environment.
 */
pub fn enable_extension() -> Result<bool, XWinError> {
  #[cfg(not(target_os = "linux"))]
  {
    Ok(false)
  }
  #[cfg(target_os = "linux")]
  {
    Ok(linux::gnome_enable_extension())
  }
}

/**
 * Disable Gnome extensions required for Linux using Gnome > 41.
 * This function will disable extension needed to correctly detect working windows with Wayland desktop environment.
 */
pub fn disable_extension() -> Result<bool, XWinError> {
  #[cfg(not(target_os = "linux"))]
  {
    Ok(false)
  }
  #[cfg(target_os = "linux")]
  {
    Ok(linux::gnome_disable_extension())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[cfg(not(target_os = "linux"))]
  use std::process::Command;
  #[cfg(not(target_os = "linux"))]
  use std::{thread, time};

  #[cfg(not(target_os = "linux"))]
  struct TestContext;

  #[cfg(not(target_os = "linux"))]
  impl TestContext {
    fn setup() -> Self {
      let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
          .args([
            "/C",
            "start",
            "microsoft-edge:https://github.com",
            "--no-first-run",
            "--restore-last-session",
          ])
          .output()
          .expect("failed to execute process")
      } else {
        Command::new("open")
          .args(["-a", "Safari", "https://github.com"])
          .output()
          .expect("failed to execute process")
      };
      println!(
        "[START] Command Status: {:?}; Command stdout: {:?}; Command stderr: {:?}",
        output.status,
        (match std::str::from_utf8(&output.stdout) {
          Ok(val) => val,
          Err(_) => "Error when convert output",
        }),
        (match std::str::from_utf8(&output.stderr) {
          Ok(val) => val,
          Err(_) => "Error when convert output",
        })
      );
      thread::sleep(time::Duration::from_secs(3));
      TestContext
    }
  }

  #[cfg(not(target_os = "linux"))]
  impl Drop for TestContext {
    fn drop(&mut self) {
      let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
          .args(["/C", "taskkill", "/f", "/im", "msedge.exe"])
          .output()
          .expect("failed to execute process")
      } else {
        Command::new("killall")
          .args(["Safari"])
          .output()
          .expect("failed to execute process")
      };
      println!(
        "[DONE] Command Status: {:?}; Command stdout: {:?}; Command stderr: {:?}",
        output.status,
        (match std::str::from_utf8(&output.stdout) {
          Ok(val) => val,
          Err(_) => "Error when convert output",
        }),
        (match std::str::from_utf8(&output.stderr) {
          Ok(val) => val,
          Err(_) => "Error when convert output",
        })
      );
      thread::sleep(time::Duration::from_secs(3));
    }
  }

  fn test_osname() -> String {
    #[cfg(target_os = "linux")]
    {
      r#"linux"#.to_owned()
    }
    #[cfg(target_os = "macos")]
    {
      r#"darwin"#.to_owned()
    }
    #[cfg(target_os = "windows")]
    {
      r#"win32"#.to_owned()
    }
  }

  fn test_struct(window_info: WindowInfo) -> Result<(), String> {
    assert_ne!(window_info.id, 0);
    assert_ne!(window_info.title, String::from(""));
    #[cfg(target_os = "linux")]
    assert_eq!(window_info.os, r#"linux"#);
    #[cfg(target_os = "macos")]
    assert_eq!(window_info.os, r#"darwin"#);
    #[cfg(target_os = "windows")]
    assert_eq!(window_info.os, r#"win32"#);
    Ok(())
  }

  #[test]
  fn test_get_active_window() -> Result<(), String> {
    let window_info = get_active_window().unwrap();
    test_struct(window_info)
  }

  #[test]
  fn test_get_open_windows() -> Result<(), String> {
    let open_windows = get_open_windows().unwrap();
    assert_ne!(open_windows.len(), 0);
    let window_info = open_windows.first().unwrap().to_owned();
    test_struct(window_info)
  }

  #[test]
  fn test_os_name() -> Result<(), String> {
    let os_name = os_name();
    assert_eq!(os_name, test_osname());
    Ok(())
  }

  #[test]
  fn test_empty_entity() -> Result<(), String> {
    let window_info = empty_entity();
    assert_eq!(window_info.id, 0);
    assert_eq!(window_info.title, String::from(""));
    assert_eq!(window_info.os, test_osname());
    Ok(())
  }

  #[test]
  fn test_get_window_icon() -> Result<(), String> {
    let window_info: &WindowInfo = &get_active_window().unwrap();
    let icon_info = get_window_icon(&window_info).unwrap();
    assert_ne!(icon_info.data, "");
    assert_ne!(icon_info.height, 0);
    assert_ne!(icon_info.width, 0);
    let open_windows = &get_open_windows().unwrap();
    assert_ne!(open_windows.len(), 0);
    let window_info = open_windows.first().unwrap().to_owned();
    let icon_info = get_window_icon(&window_info).unwrap();
    assert_ne!(icon_info.data, "");
    assert_ne!(icon_info.height, 0);
    assert_ne!(icon_info.width, 0);
    Ok(())
  }

  #[cfg(not(target_os = "linux"))]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_get_brower_url() -> Result<(), String> {
    #[allow(unused)]
    let _context = TestContext::setup();
    let open_windows = &get_open_windows().unwrap();
    assert_ne!(open_windows.len(), 0);
    let window_info = open_windows.first().unwrap().to_owned();
    let url = get_browser_url(&window_info).unwrap();
    println!("URL: {:?}; process: {:?}", url, window_info.info.name);
    assert!(url.starts_with("http"));
    let window_info = &get_active_window().unwrap().to_owned();
    let url = get_browser_url(&window_info).unwrap();
    println!("URL: {:?}; process: {:?}", url, window_info.info.name);
    assert!(url.starts_with("http"));
    Ok(())
  }

  #[cfg(target_os = "linux")]
  #[test]
  fn test_get_brower_url() -> Result<(), String> {
    let open_windows = &get_open_windows().unwrap();
    assert_ne!(open_windows.len(), 0);
    let window_info = open_windows.first().unwrap().to_owned();
    let url = get_browser_url(&window_info).unwrap();
    assert!(url.eq("URL recovery not supported on Linux distribution!"));
    let window_info = &get_active_window().unwrap().to_owned();
    let url = get_browser_url(&window_info).unwrap();
    assert!(url.eq("URL recovery not supported on Linux distribution!"));
    Ok(())
  }

  #[cfg(all(feature = "macos_permission", target_os = "macos"))]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_check_screen_record_permission() -> Result<(), String> {
    use macos::permission::check_screen_record_permission;
    let value = check_screen_record_permission();
    assert_eq!(value, true);
    Ok(())
  }

  #[cfg(all(feature = "macos_permission", target_os = "macos"))]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_request_screen_record_permission() -> Result<(), String> {
    use macos::permission::request_screen_record_permission;
    let value = request_screen_record_permission();
    assert_eq!(value, true);
    Ok(())
  }
}
