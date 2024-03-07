#![allow(unused_imports)]

use std::{
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
};

use crate::{
  common::{api::API, x_win_struct::window_info::WindowInfo},
  linux::api::common_api::{get_window_memory_usage, get_window_path_name},
};

use super::common_api::{get_gnome_version, init_entity};

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
    let mut result: WindowInfo = init_entity();

    result
  }

  fn get_open_windows(&self) -> Vec<WindowInfo> {
    let mut results: Vec<WindowInfo> = Vec::new();

    results
  }
}