#![deny(unused_imports)]

use super::x_win_struct::{window_info::WindowInfo, window_position::WindowPosition, process_info::ProcessInfo, usage_info::UsageInfo};

pub trait API {
  /**
   * Return information of current active Window
   */
  fn get_active_window(&self) -> WindowInfo;

  /**
   * Return Array of open windows information
   */
  fn get_open_windows(&self) -> Vec<WindowInfo>;
}

/**
 * To know the os
 */
#[cfg(target_os = "linux")]
pub fn os_name() -> String {
  r#"linux"#.to_owned()
}

/**
 * To know the os
 */
#[cfg(target_os = "macos")]
pub fn os_name() -> String {
  r#"darwin"#.to_owned()
}

/**
 * To know the os
 */
#[cfg(target_os = "windows")]
pub fn os_name() -> String {
  r#"win32"#.to_owned()
}

pub fn empty_entity() -> WindowInfo {
  WindowInfo {
    id: 0,
    os: os_name().to_owned(),
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
    url: "".to_string(),
  }
}
