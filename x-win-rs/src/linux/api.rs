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

use crate::common::{
  api::Api,
  result::Result,
  x_win_struct::{icon_info::IconInfo, window_info::WindowInfo},
};

pub trait APIGnome {
  fn install_extension() -> Result<bool>;
  fn uninstall_extension() -> Result<bool>;
  fn enable_extension() -> Result<bool>;
  fn disable_extension() -> Result<bool>;
  fn is_installed_extension() -> Result<bool>;
  fn is_enabled_extension() -> Result<bool>;
}

pub struct LinuxAPI {}

/**
 * Impl. for windows system
 */
impl Api for LinuxAPI {
  fn get_active_window(&self) -> crate::common::result::Result<WindowInfo> {
    Ok(match is_wayland_desktop() {
      true => (WaylandApi {}).get_active_window()?,
      false => (X11Api {}).get_active_window()?,
    })
  }

  fn get_open_windows(&self) -> crate::common::result::Result<Vec<WindowInfo>> {
    Ok(match is_wayland_desktop() {
      true => (WaylandApi {}).get_open_windows()?,
      false => (X11Api {}).get_open_windows()?,
    })
  }

  fn get_app_icon(&self, window_info: &WindowInfo) -> crate::common::result::Result<IconInfo> {
    Ok(match is_wayland_desktop() {
      true => (WaylandApi {}).get_app_icon(window_info)?,
      false => (X11Api {}).get_app_icon(window_info)?,
    })
  }

  fn get_browser_url(&self, window_info: &WindowInfo) -> crate::common::result::Result<String> {
    Ok(match is_wayland_desktop() {
      true => (WaylandApi {}).get_browser_url(window_info)?,
      false => (X11Api {}).get_browser_url(window_info)?,
    })
  }
}

impl APIGnome for LinuxAPI {
  fn install_extension() -> Result<bool> {
    if is_wayland_desktop() {
      WaylandApi::install_extension()
    } else {
      Ok(false)
    }
  }

  fn uninstall_extension() -> Result<bool> {
    if is_wayland_desktop() {
      WaylandApi::uninstall_extension()
    } else {
      Ok(false)
    }
  }

  fn enable_extension() -> Result<bool> {
    if is_wayland_desktop() {
      WaylandApi::enable_extension()
    } else {
      Ok(false)
    }
  }

  fn disable_extension() -> Result<bool> {
    if is_wayland_desktop() {
      WaylandApi::disable_extension()
    } else {
      Ok(false)
    }
  }

  fn is_installed_extension() -> Result<bool> {
    if is_wayland_desktop() {
      WaylandApi::disable_extension()
    } else {
      Ok(false)
    }
  }

  fn is_enabled_extension() -> Result<bool> {
    if is_wayland_desktop() {
      WaylandApi::is_enabled_extension()
    } else {
      Ok(false)
    }
  }
}
