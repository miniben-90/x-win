#![deny(unsafe_op_in_unsafe_fn)]
// #![deny(clippy::all)]
#![allow(unused_imports)]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate core;

use common::{x_win_struct::window_info::WindowInfo, api::API};
use napi::Result;

mod common;

#[cfg(target_os = "windows")]
mod win32;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
use win32::init_platform_api;

#[cfg(target_os = "linux")]
use linux::init_platform_api;

#[cfg(target_os = "macos")]
use macos::init_platform_api;


#[macro_use]
extern crate napi_derive;

#[napi]
pub fn active_window() -> Result<WindowInfo> {
  let api = init_platform_api();
  api.get_active_window()
}

#[napi]
pub fn open_windows() -> Result<Vec<WindowInfo>> {
  let api = init_platform_api();
  api.get_open_windows()
}
