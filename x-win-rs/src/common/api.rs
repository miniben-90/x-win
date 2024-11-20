#![deny(unused_imports)]

use super::x_win_struct::{
  icon_info::IconInfo, process_info::ProcessInfo, usage_info::UsageInfo, window_info::WindowInfo,
  window_position::WindowPosition,
};

pub trait Api {
  /**
   * Return information of current active Window
   */
  fn get_active_window(&self) -> WindowInfo;

  /**
   * Return Array of open windows information
   */
  fn get_open_windows(&self) -> Vec<WindowInfo>;

  /**
   * Return a base64 icon from window_info.info.path
   */
  fn get_app_icon(&self, window_info: &WindowInfo) -> IconInfo;

  /**
   * Return a String if the window is a browser and can recover url from it (Work only with Windows 10/11 and Darwin systems)
   */
  fn get_browser_url(&self, window_info: &WindowInfo) -> String;
}

/**
 * To know the os
 */
pub fn os_name() -> String {
  #[cfg(target_os = "windows")]
  {
    r#"win32"#.to_owned()
  }

  #[cfg(target_os = "linux")]
  {
    r#"linux"#.to_owned()
  }

  #[cfg(target_os = "macos")]
  {
    r#"darwin"#.to_owned()
  }
}

pub fn empty_entity() -> WindowInfo {
  WindowInfo {
    id: 0,
    os: os_name(),
    title: "".to_string(),
    position: WindowPosition {
      x: 0,
      y: 0,
      width: 0,
      height: 0,
      is_full_screen: false,
    },
    info: ProcessInfo {
      process_id: 0,
      path: "".to_string(),
      name: "".to_string(),
      exec_name: "".to_string(),
    },
    usage: UsageInfo { memory: 0 },
  }
}

pub fn empty_icon() -> IconInfo {
  IconInfo {
    data: String::from(""),
    height: 0,
    width: 0,
  }
}
