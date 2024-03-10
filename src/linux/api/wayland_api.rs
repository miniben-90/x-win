#![allow(unused_imports)]

use std::{
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
};

use crate::{
  common::{api::API, x_win_struct::window_info::WindowInfo},
  linux::api::common_api::{get_window_memory_usage, get_window_path_name},
};

use super::{common_api::{get_gnome_version, init_entity}, gnome_shell, wayland_eval_api, wayland_new_api};

use once_cell::sync::Lazy;

struct GnomeVersion {
  version: String,
  use_eval: bool,
}

impl GnomeVersion {
    fn new() -> Self {
      let version = get_gnome_version();
      let ver: u32 = version.split(".").collect::<Vec<&str>>()[0].parse().unwrap_or(999);
      let use_eval = ver < 41;
      Self { version, use_eval }
    }
}

static GNOME_SINGLETON: Lazy<Mutex<GnomeVersion>> = Lazy::new(|| Mutex::new(GnomeVersion::new()));

/**
 * Struct to use similar as API to get active window and open windows for XOrg desktop
 */
pub struct WaylandApi {}

/**
 * Impl. for windows system
 */
impl API for WaylandApi {
  fn get_active_window(&self) -> WindowInfo {
    let gnome_singleton = GNOME_SINGLETON.lock().unwrap();
    if gnome_singleton.use_eval {
      wayland_eval_api::get_active_window()
    } else {
      wayland_new_api::get_active_window()
    }
  }

  fn get_open_windows(&self) -> Vec<WindowInfo> {
    let gnome_singleton = GNOME_SINGLETON.lock().unwrap();
    if gnome_singleton.use_eval {
      wayland_eval_api::get_open_windows()
    } else {
      wayland_new_api::get_open_windows()
    }
  }
}