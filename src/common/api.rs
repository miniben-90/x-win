#![deny(unused_imports)]

use super::x_win_struct::window_info::WindowInfo;

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