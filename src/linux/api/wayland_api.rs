#![allow(unused_imports)]

use std::{
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
};

use zbus::{blocking::Connection, dbus_interface, dbus_proxy, fdo};

use crate::{
  common::{api::API, x_win_struct::window_info::WindowInfo},
  linux::api::common_api::{get_window_memory_usage, get_window_path_name},
};

use super::common_api::init_entity;

const XWIN_WAYLAND_EXTENSION_SCRIPT: &str = include_str!("wayland-extension.js");

// #[dbus_interface(name = "org.gnome.Shell.Extensions.XWinWaylandExtension")]
// impl trait WaylandExtension {
//   fn get_active_window() -> String;
//   fn get_open_windows() -> String;
// }

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
