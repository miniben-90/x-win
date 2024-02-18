#![deny(unused_imports)]

use std::{
  fs::{read_link, File},
  io::Read,
};

use crate::common::{x_win_struct::window_info::WindowInfo, api::empty_entity};

/**
 * To know the os
 */
pub fn os_name() -> String {
  r#"linux"#.to_owned()
}

/**
 * To known if desktop run with wayland or not
 */
pub fn is_wayland_desktop() -> bool {
  std::env::var("WAYLAND_DISPLAY")
    .map(|val| !val.is_empty())
    .unwrap_or(false)
}

/**
 * Get usage memory of window from proc
 */
pub fn get_window_memory_usage(pid: u32) -> u32 {
  let mut statm_file = File::open(format!("/proc/{}/statm", pid)).unwrap();
  let mut statm_content = String::new();
  statm_file.read_to_string(&mut statm_content).unwrap();
  let statm_parts: Vec<&str> = statm_content.split(" ").collect();
  return statm_parts[0].parse().unwrap();
}

/**
 * Recover path and name of application from proc
 */
pub fn get_window_path_name(pid: u32) -> (String, String) {
  let executable_path = read_link(format!("/proc/{}/exe", pid)).unwrap();
  let path = executable_path.display().to_string();
  let name = executable_path.file_name().unwrap();
  let name = name.to_string_lossy().to_string();
  return (path, name);
}

pub fn init_entity() -> WindowInfo {
  let mut window_info: WindowInfo = empty_entity();
  window_info.os = os_name();
  window_info
}
