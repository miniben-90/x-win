#![allow(unused_imports)]

use std::{ops::Deref, path::{Path, PathBuf}};

use crate::{
  common::{api::API, x_win_struct::window_info::WindowInfo},
  linux::api::{
    common_api::{get_window_memory_usage, get_window_path_name},
    gnome_shell::GNOME_XWIN_EXTENSION_FOLDER_PATH,
  },
};

use super::{
  common_api::{get_gnome_version, init_entity},
  gnome_shell::{self, GNOME_SINGLETON}, wayland_eval_api, wayland_extension_api, APIGnome,
};

fn gnome_use_eval() -> bool {
  let gnome_singleton = GNOME_SINGLETON.lock().unwrap();
  let use_eval: bool = gnome_singleton.use_eval.clone();
  let _ = gnome_singleton.deref();
  use_eval
}

/**
 * Struct to use similar as API to get active window and open windows for XOrg desktop
 */
pub struct WaylandApi {}

/**
 * Impl. for Linux system
 */
impl API for WaylandApi {
  fn get_active_window(&self) -> WindowInfo {
    if gnome_use_eval() {
      wayland_eval_api::get_active_window()
    } else {
      wayland_extension_api::get_active_window()
    }
  }

  fn get_open_windows(&self) -> Vec<WindowInfo> {
    if gnome_use_eval() {
      wayland_eval_api::get_open_windows()
    } else {
      wayland_extension_api::get_open_windows()
    }
  }
}

impl APIGnome for WaylandApi {
  fn install_extension() -> bool {
    if !gnome_use_eval() {
      wayland_extension_api::install_extension()
    } else {
      false
    }
  }

  fn uninstall_extension() -> bool {
    if !gnome_use_eval() {
      wayland_extension_api::uninstall_extension()
    } else {
      false
    }
  }

  fn enable_extension() -> bool {
    if !gnome_use_eval() {
      wayland_extension_api::enable_extension()
    } else {
      false
    }
  }

  fn disable_extension() -> bool {
    if !gnome_use_eval() {
      wayland_extension_api::disable_extension()
    } else {
      false
    }
  }
}
