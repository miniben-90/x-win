#![deny(unused_imports)]

use std::{
  fs::{read_link, File},
  io::Read,
};

use std::process::Command;

use crate::common::{api::empty_entity, result::Result, x_win_struct::window_info::WindowInfo};

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
pub fn get_window_memory_usage(pid: u32) -> Result<u32> {
  let mut statm_file = File::open(format!("/proc/{pid}/statm"))?;
  let mut statm_content = String::new();
  statm_file.read_to_string(&mut statm_content)?;
  let statm_parts: Vec<&str> = statm_content.split(" ").collect();
  let memory_usage = statm_parts[0].parse()?;
  Ok(memory_usage)
}

/**
 * Recover path and name of application from proc
 */
pub fn get_window_path_name(pid: u32) -> Result<(String, String)> {
  let executable_path = read_link(format!("/proc/{pid}/exe"))?;
  let path = executable_path.display().to_string();
  let name = match executable_path.file_name() {
    Some(file_name) => file_name.to_string_lossy().to_string(),
    None => executable_path.to_string_lossy().to_string(),
  };
  Ok((path, name))
}

pub fn init_entity() -> WindowInfo {
  let mut window_info: WindowInfo = empty_entity();
  window_info.os = os_name();
  window_info
}

pub fn get_gnome_version() -> String {
  if let Ok(output) = Command::new("gnome-shell").arg("--version").output() {
    if output.status.success() {
      let stdout = String::from_utf8_lossy(&output.stdout);
      let version = stdout.split_whitespace().nth(2).unwrap_or("999");
      return version.to_owned();
    }
  }
  "999".into()
}

pub fn get_browser_url() -> String {
  String::from("URL recovery not supported on Linux distribution!")
}
