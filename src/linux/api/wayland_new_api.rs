use crate::common::x_win_struct::window_info::WindowInfo;

use super::common_api::init_entity;

pub fn get_active_window() -> WindowInfo {
  init_entity()
}

pub fn get_open_windows() -> Vec<WindowInfo> {
  vec![]
}