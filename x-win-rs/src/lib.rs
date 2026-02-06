#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::all)]

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
/// Handle screen record permission
/// <div class="warning">important Work only with macOS 10.15+</div>
/// To use this function you need to add `macos_permission` feature
pub use macos::permission;

pub use common::{
  api::{empty_entity, os_name, Api},
  result::Result,
  x_win_struct::{
    icon_info::IconInfo, process_info::ProcessInfo, usage_info::UsageInfo, window_info::WindowInfo,
    window_position::WindowPosition,
  },
};

/**
 * Recover icon of window.
 * Return `IconInfo`
 */
pub fn get_window_icon(window_info: &WindowInfo) -> Result<IconInfo> {
  let api = init_platform_api();
  let icon_info = api.get_app_icon(window_info)?;
  Ok(icon_info)
}

/**
 * Recover browser url of window.
 * Return `String`
 */
pub fn get_browser_url(window_info: &WindowInfo) -> Result<String> {
  let api = init_platform_api();
  let browser_url = api.get_browser_url(window_info)?;
  Ok(browser_url)
}

/**
 * Retrieve information the about currently active window.
 * Return `WindowInfo` containing details about a specific active window.
 */
pub fn get_active_window() -> Result<WindowInfo> {
  let api = init_platform_api();
  let active_window = api.get_active_window()?;
  Ok(active_window)
}

/**
 * Retrieve information about the currently open windows.
 * Return `Vec<WindowInfo>` each containing details about a specific open window.
 */
pub fn get_open_windows() -> Result<Vec<WindowInfo>> {
  let api = init_platform_api();
  let open_windows = api.get_open_windows()?;
  Ok(open_windows)
}

/**
 * Install "@mininben90/x-win" Gnome extension required for Linux using Gnome > 41.
 * This function will write extension files needed to correctly detect working windows with Wayland desktop environment.
 * **Restart session will be require to install the gnome extension.**
 */
pub fn install_extension() -> Result<bool> {
  #[cfg(not(target_os = "linux"))]
  {
    Ok(false)
  }
  #[cfg(target_os = "linux")]
  {
    linux::gnome_install_extension()
  }
}

/**
 * Uninstall "@mininben90/x-win" Gnome extension.
 * This function will disable and remove extension files.
 * **Restart session will be require to remove the gnome extension.**
 */
pub fn uninstall_extension() -> Result<bool> {
  #[cfg(not(target_os = "linux"))]
  {
    Ok(false)
  }
  #[cfg(target_os = "linux")]
  {
    linux::gnome_uninstall_extension()
  }
}

/**
 * Enable Gnome extension required for Linux using Gnome > 41.
 * This function will enable extension needed to correctly detect working windows with Wayland desktop environment.
 */
pub fn enable_extension() -> Result<bool> {
  #[cfg(not(target_os = "linux"))]
  {
    Ok(false)
  }
  #[cfg(target_os = "linux")]
  {
    linux::gnome_enable_extension()
  }
}

/**
 * Disable Gnome extension required for Linux using Gnome > 41.
 * This function will disable extension needed to correctly detect working windows with Wayland desktop environment.
 */
pub fn disable_extension() -> Result<bool> {
  #[cfg(not(target_os = "linux"))]
  {
    Ok(false)
  }
  #[cfg(target_os = "linux")]
  {
    linux::gnome_disable_extension()
  }
}

/**
 * Return true of false if gnome extension is enabled for Linux using Gnome > 41.
 * This function will return true or false if the extension is set to enabled on extension info. Working only with Wayland windows manager.
 */
pub fn is_enabled_extension() -> Result<bool> {
  #[cfg(not(target_os = "linux"))]
  {
    Ok(false)
  }
  #[cfg(target_os = "linux")]
  {
    linux::gnome_is_enabled_extension()
  }
}

/**
 * Return true of false the extensions is installed for Linux using Gnome > 41.
 * This function will return true or false if the extension is correctly installed. Working only with Wayland windows manager.
 */
pub fn is_installed_extension() -> Result<bool> {
  #[cfg(not(target_os = "linux"))]
  {
    Ok(false)
  }
  #[cfg(target_os = "linux")]
  {
    linux::gnome_is_installed_extension()
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
        std::str::from_utf8(&output.stdout).unwrap_or("Error when convert output"),
        std::str::from_utf8(&output.stderr).unwrap_or("Error when convert stderr")
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
        std::str::from_utf8(&output.stdout).unwrap_or("Error when convert output"),
        std::str::from_utf8(&output.stderr).unwrap_or("Error when convert stderr")
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

  fn test_struct(window_info: WindowInfo) -> Result<()> {
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
  fn test_get_active_window() -> Result<()> {
    match get_active_window() {
      Ok(window_info) => test_struct(window_info),
      Err(err) => Err(err),
    }
  }

  #[test]
  fn test_get_open_windows() -> Result<()> {
    match get_open_windows() {
      Ok(open_windows) => {
        assert_ne!(open_windows.len(), 0);

        if let Some(window_info) = open_windows.first() {
          test_struct(window_info.to_owned())
        } else {
          Err("No open window".into())
        }
      }
      Err(err) => Err(err),
    }
  }

  #[test]
  fn test_os_name() -> Result<()> {
    let os_name = os_name();
    assert_eq!(os_name, test_osname());
    Ok(())
  }

  #[test]
  fn test_empty_entity() -> Result<()> {
    let window_info = empty_entity();
    assert_eq!(window_info.id, 0);
    assert_eq!(window_info.title, String::from(""));
    assert_eq!(window_info.os, test_osname());
    Ok(())
  }

  #[test]
  fn test_get_window_icon_from_active_window() -> Result<()> {
    match get_active_window() {
      Ok(window_info) => match get_window_icon(&window_info) {
        Ok(icon_info) => {
          assert_ne!(icon_info.data, "");
          assert_ne!(icon_info.height, 0);
          assert_ne!(icon_info.width, 0);
          Ok(())
        }
        Err(err) => Err(err),
      },
      Err(err) => Err(err),
    }
  }

  #[test]
  fn test_get_window_icon_from_open_windows() -> Result<()> {
    match get_open_windows() {
      Ok(open_windows) => match open_windows.first() {
        Some(window_info) => match get_window_icon(&window_info) {
          Ok(icon_info) => {
            assert_ne!(icon_info.data, "");
            assert_ne!(icon_info.height, 0);
            assert_ne!(icon_info.width, 0);
            Ok(())
          }
          Err(err) => Err(err),
        },
        None => Err("No open window".into()),
      },
      Err(err) => Err(err),
    }
  }

  #[cfg(not(target_os = "linux"))]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_get_browser_url_from_active_window() -> Result<()> {
    #[allow(unused)]
    let _context: TestContext = TestContext::setup();
    match get_active_window() {
      Ok(window_info) => match get_browser_url(&window_info) {
        Ok(url) => {
          assert!(url.starts_with("http"));
          Ok(())
        }
        Err(err) => Err(err),
      },
      Err(err) => Err(err),
    }
  }

  #[cfg(not(target_os = "linux"))]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_get_browser_url_from_open_windows() -> Result<()> {
    #[allow(unused)]
    let _context = TestContext::setup();
    match get_open_windows() {
      Ok(open_windows) => {
        assert_ne!(open_windows.len(), 0);
        match open_windows.first() {
          Some(window_info) => match get_browser_url(window_info) {
            Ok(url) => {
              assert!(url.starts_with("http"));
              Ok(())
            }
            Err(err) => Err(err),
          },
          None => Err("No open window".into()),
        }
      }
      Err(err) => Err(err),
    }
  }

  #[cfg(target_os = "linux")]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_get_browser_url_from_active_window() -> Result<()> {
    #[allow(unused)]
    match get_active_window() {
      Ok(window_info) => match get_browser_url(&window_info) {
        Ok(url) => {
          assert!(url.eq("URL recovery not supported on Linux distribution!"));
          Ok(())
        }
        Err(err) => Err(err),
      },
      Err(err) => Err(err),
    }
  }

  #[cfg(target_os = "linux")]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_get_browser_url_from_open_windows() -> Result<()> {
    #[allow(unused)]
    match get_open_windows() {
      Ok(open_windows) => {
        assert_ne!(open_windows.len(), 0);
        match open_windows.first() {
          Some(window_info) => match get_browser_url(window_info) {
            Ok(url) => {
              assert!(url.eq("URL recovery not supported on Linux distribution!"));
              Ok(())
            }
            Err(err) => Err(err),
          },
          None => Err("No open window".into()),
        }
      }
      Err(err) => Err(err),
    }
  }

  #[cfg(all(feature = "macos_permission", target_os = "macos"))]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_check_screen_record_permission() -> Result<()> {
    use macos::permission::check_screen_record_permission;
    let value = check_screen_record_permission();
    assert_eq!(value, true);
    Ok(())
  }

  #[cfg(all(feature = "macos_permission", target_os = "macos"))]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_request_screen_record_permission() -> Result<()> {
    use macos::permission::request_screen_record_permission;
    let value = request_screen_record_permission();
    assert_eq!(value, true);
    Ok(())
  }

  #[cfg(target_os = "linux")]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_install_extension() -> Result<()> {
    let value = install_extension()?;
    assert_eq!(value, true);
    Ok(())
  }

  #[cfg(target_os = "linux")]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_uninstall_extension() -> Result<()> {
    let value = uninstall_extension()?;
    assert_eq!(value, true);
    Ok(())
  }

  #[cfg(target_os = "linux")]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_enable_extension() -> Result<()> {
    let value = enable_extension()?;
    assert_eq!(value, true);
    Ok(())
  }

  #[cfg(target_os = "linux")]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_disable_extension() -> Result<()> {
    let value = disable_extension()?;
    assert_eq!(value, true);
    Ok(())
  }

  #[cfg(target_os = "linux")]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_is_enabled_extension() -> Result<()> {
    let value = is_enabled_extension()?;
    assert_eq!(value, true);
    Ok(())
  }

  #[cfg(target_os = "linux")]
  #[test]
  #[ignore = "Not working on ci/cd"]
  fn test_is_installed_extension() -> Result<()> {
    let value = is_installed_extension()?;
    assert_eq!(value, true);
    Ok(())
  }
}
