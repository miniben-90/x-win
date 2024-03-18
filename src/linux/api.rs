#![deny(unused_imports)]

mod x11_api;
mod common_api;
mod wayland_api;
mod gnome_shell;
mod wayland_eval_api;
mod wayland_extension_api;
mod wayland_extension45_api;

use x11_api::X11Api;
use wayland_api::WaylandApi;
use common_api::is_wayland_desktop;

use crate::common::{api::API, x_win_struct::window_info::WindowInfo};

pub trait APIGnome {
  fn install_extension() -> ();
  fn uninstall_extension() -> ();
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
    fn install_extension() -> () {
      if is_wayland_desktop() {
        WaylandApi::install_extension()
      } else {
        ()
      }
    }

    fn uninstall_extension() -> () {
      if is_wayland_desktop() {
        WaylandApi::uninstall_extension()
      } else {
        ()
      }
    }
}