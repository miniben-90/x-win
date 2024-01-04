#![deny(unsafe_op_in_unsafe_fn)]
// #![deny(clippy::all)]
#![allow(unused_imports)]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate core;

use common::{api::API, thread::ThreadManager, x_win_struct::window_info::WindowInfo};
use napi::{Env, JsFunction, JsNumber, JsObject, JsUndefined, Result};
use napi_derive::napi;

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

use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use std::{
  borrow::Borrow,
  collections::{hash_map::DefaultHasher, HashMap},
  hash::{Hash, Hasher},
  thread,
  time::Duration,
};

use crate::common::x_win_struct::{
  process_info::ProcessInfo, usage_info::UsageInfo, window_position::WindowPosition,
};

use once_cell::sync::Lazy;
use std::sync::{Mutex, OnceLock};

static THREAD_MANAGER: Lazy<Mutex<ThreadManager>> = Lazy::new(|| Mutex::new(ThreadManager::new()));

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

#[napi(ts_args_type = "callback: (info: WindowInfo) => void")]
pub fn subscribe_active_window(callback: JsFunction) -> Result<u32> {
  let api = init_platform_api();
  let tsfn: ThreadsafeFunction<WindowInfo, ErrorStrategy::Fatal> = callback
    .create_threadsafe_function(
      0,
      |ctx: napi::threadsafe_function::ThreadSafeCallContext<WindowInfo>| Ok(vec![ctx.value]),
    )?;

  let tsfn_clone: ThreadsafeFunction<WindowInfo, ErrorStrategy::Fatal> = tsfn.clone();

  let thread_manager = THREAD_MANAGER.lock().unwrap();

  let id = thread_manager.start_thread(move |receiver| {
    let mut current_window: WindowInfo = WindowInfo {
      id: 0,
      os: "".to_string(),
      title: "".to_string(),
      position: WindowPosition {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
      },
      info: ProcessInfo {
        process_id: 0,
        path: "".to_string(),
        name: "".to_string(),
        exec_name: "".to_string(),
      },
      usage: UsageInfo { memory: 0 },
      url: "".to_string(),
    };
    loop {
      match receiver.try_recv() {
        Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => {
          break;
        }
        _ => {
          let new_current_window = api.get_active_window().unwrap();
          if new_current_window.id.ne(&current_window.id)
            || new_current_window.title.ne(&current_window.title)
            || new_current_window
              .info
              .process_id
              .ne(&current_window.info.process_id)
          {
            current_window = new_current_window.clone();
            tsfn_clone.call(new_current_window, ThreadsafeFunctionCallMode::Blocking);
          }
          thread::sleep(Duration::from_millis(100));
        }
      }
    }
  });

  Ok(id.unwrap())
}

#[napi]
pub fn unsubscribe_active_window(thread_id: u32) -> Result<()> {
  THREAD_MANAGER.lock().unwrap().stop_thread(thread_id).unwrap();
  Ok(())
}

#[napi]
pub fn unsubscribe_all_active_window() -> Result<()> {
  THREAD_MANAGER.lock().unwrap().stop_all_threads().unwrap();
  Ok(())
}
