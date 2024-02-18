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

pub fn empty_entity() -> WindowInfo {
  WindowInfo {
    id: 0,
    os: "".to_string(),
    title: "".to_string(),
    position: WindowPosition {
      x: 0,
      y: 0,
      width: 0,
      height: 0,
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
