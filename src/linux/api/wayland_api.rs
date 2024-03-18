#![allow(unused_imports)]

use std::{
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
};

use crate::{
  common::{api::API, x_win_struct::window_info::WindowInfo},
  linux::api::{
    common_api::{get_window_memory_usage, get_window_path_name},
    gnome_shell::GNOME_XWIN_EXTENSION_FOLDER_PATH,
  },
};

use super::{
  common_api::{get_gnome_version, init_entity},
  gnome_shell, wayland_eval_api, wayland_extension45_api, wayland_extension_api, APIGnome,
};

use once_cell::sync::Lazy;

struct GnomeVersion {
  version: u32,
  use_eval: bool,
}

impl GnomeVersion {
  fn new() -> Self {
    let version = get_gnome_version();
    let version: u32 = version.split(".").collect::<Vec<&str>>()[0]
      .parse()
      .unwrap_or(999);
    let use_eval = version < 41;
    Self { use_eval, version }
  }
}

static GNOME_SINGLETON: Lazy<Mutex<GnomeVersion>> = Lazy::new(|| Mutex::new(GnomeVersion::new()));

/**
 * Struct to use similar as API to get active window and open windows for XOrg desktop
 */
pub struct WaylandApi {}

/**
 * Impl. for Linux system
 */
impl API for WaylandApi {
  fn get_active_window(&self) -> WindowInfo {
    let gnome_singleton = GNOME_SINGLETON.lock().unwrap();
    if gnome_singleton.use_eval {
      wayland_eval_api::get_active_window()
    } else if gnome_singleton.version.lt(&45) {
      wayland_extension_api::get_active_window()
    } else {
      wayland_extension45_api::get_active_window()
    }
  }

  fn get_open_windows(&self) -> Vec<WindowInfo> {
    let gnome_singleton = GNOME_SINGLETON.lock().unwrap();
    if gnome_singleton.use_eval {
      wayland_eval_api::get_open_windows()
    } else if gnome_singleton.version.lt(&45) {
      wayland_extension_api::get_open_windows()
    } else {
      wayland_extension45_api::get_open_windows()
    }
  }
}

impl APIGnome for WaylandApi {
  fn install_extension() -> () {
    let gnome_singleton = GNOME_SINGLETON.lock().unwrap();
    if !gnome_singleton.use_eval {
      if gnome_singleton.version.lt(&45) {
        wayland_extension_api::install_extension()
      } else {
        wayland_extension45_api::install_extension()
      }
    } else {
      ()
    }
  }

  fn uninstall_extension() -> () {
    let gnome_singleton = GNOME_SINGLETON.lock().unwrap();
    if !gnome_singleton.use_eval {
      if gnome_singleton.version.lt(&45) {
        wayland_extension_api::uninstall_extension()
      } else {
        wayland_extension45_api::uninstall_extension()
      }
    } else {
      ()
    }
  }
}
