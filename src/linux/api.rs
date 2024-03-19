#![deny(unused_imports)]

mod common_api;
mod gnome_shell;
mod wayland_api;
mod wayland_eval_api;
mod wayland_extension_api;
mod x11_api;

use common_api::is_wayland_desktop;
use wayland_api::WaylandApi;
use x11_api::X11Api;

use crate::common::{api::API, x_win_struct::window_info::WindowInfo};

pub trait APIGnome {
  fn install_extension() -> bool;
  fn uninstall_extension() -> bool;
  fn enable_extension() -> bool;
  fn disable_extension() -> bool;
}

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

impl APIGnome for LinuxAPI {
  fn install_extension() -> bool {
    if is_wayland_desktop() {
      WaylandApi::install_extension()
    } else {
      false
    }
  }

  fn uninstall_extension() -> bool {
    if is_wayland_desktop() {
      WaylandApi::uninstall_extension()
    } else {
      false
    }
  }

  fn enable_extension() -> bool {
    if is_wayland_desktop() {
      WaylandApi::enable_extension()
    } else {
      false
    }
  }

  fn disable_extension() -> bool {
    if is_wayland_desktop() {
      WaylandApi::disable_extension()
    } else {
      false
    }
  }
}
