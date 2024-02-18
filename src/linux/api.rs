#![deny(unused_imports)]

mod x11_api;
mod common_api;
mod wayland_api;
mod gnome_shell_extension;

use x11_api::X11Api;
use wayland_api::WaylandApi;
use common_api::is_wayland_desktop;

use crate::common::{api::API, x_win_struct::window_info::WindowInfo};

pub struct LinuxAPI {}

/**
 * Impl. for windows system
 */
impl API for LinuxAPI {
  fn get_active_window(&self) -> WindowInfo {
    if is_wayland_desktop() {
      (WaylandApi {}).get_active_window()
    } else {
      (X11Api {}).get_active_window()
    }
  }

  fn get_open_windows(&self) -> Vec<WindowInfo> {
    if is_wayland_desktop() {
      (WaylandApi {}).get_open_windows()
    } else {
      (X11Api {}).get_open_windows()
    }
  }
}
