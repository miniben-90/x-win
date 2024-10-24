#![deny(unused_imports)]

use super::{process_info::ProcessInfo, usage_info::UsageInfo, window_position::WindowPosition};

/**
 * Struct to store all informations of the window
 */
#[derive(Debug, Clone)]
#[napi(constructor)]
pub struct WindowInfo {
  pub id: u32,
  pub os: String,
  pub title: String,
  pub position: WindowPosition,
  pub info: ProcessInfo,
  pub usage: UsageInfo,
}

#[napi]
impl WindowInfo {
  pub fn new(
    id: u32,
    os: String,
    title: String,
    position: WindowPosition,
    info: ProcessInfo,
    usage: UsageInfo,
  ) -> Self {
    Self {
      id,
      os,
      title,
      position,
      info,
      usage,
    }
  }
}

impl From<x_win::WindowInfo> for WindowInfo {
  fn from(value: x_win::WindowInfo) -> Self {
    WindowInfo {
      id: value.id,
      info: value.info.into(),
      os: value.os,
      title: value.title,
      position: value.position.into(),
      usage: value.usage.into(),
    }
  }
}

impl From<WindowInfo> for x_win::WindowInfo {
  fn from(value: WindowInfo) -> Self {
    x_win::WindowInfo {
      id: value.id,
      info: value.info.into(),
      os: value.os,
      title: value.title,
      position: value.position.into(),
      usage: value.usage.into(),
    }
  }
}
