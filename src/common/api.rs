#![deny(unused_imports)]

use napi::Result;

use super::x_win_struct::window_info::WindowInfo;

pub trait API {
  /**
   * Return information of current active Window
   */
  fn get_active_window(&self) -> Result<WindowInfo>;

  /**
   * Return Array of open windows information
   */
  fn get_open_windows(&self) -> Result<Vec<WindowInfo>>;
}