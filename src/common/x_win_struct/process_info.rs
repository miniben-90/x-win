#![deny(unused_imports)]

/**
 * Struct to store process information of the window
 */
#[derive(Debug, Clone)]
#[napi(object)]
#[repr(C)]
pub struct ProcessInfo {
  pub process_id: u32,
  pub path: String,
  pub name: String,
  pub exec_name: String,
}

impl ProcessInfo {
  pub fn new(process_id: u32, path: String, name: String, exec_name: String) -> Self {
    Self {
      process_id,
      path,
      name,
      exec_name,
    }
  }
}

impl From<x_win::ProcessInfo> for ProcessInfo {
  fn from(value: x_win::ProcessInfo) -> Self {
    ProcessInfo {
      exec_name: value.exec_name,
      name: value.name,
      path: value.path,
      process_id: value.process_id,
    }
  }
}

impl From<ProcessInfo> for x_win::ProcessInfo {
  fn from(value: ProcessInfo) -> Self {
    x_win::ProcessInfo {
      exec_name: value.exec_name,
      name: value.name,
      path: value.path,
      process_id: value.process_id,
    }
  }
}
