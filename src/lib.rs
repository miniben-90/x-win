#![deny(unsafe_op_in_unsafe_fn)]
//#![deny(clippy::all)]
//#![allow(unused_imports)]

mod common;
mod error;

use common::{
  thread::ThreadManager,
  x_win_struct::{icon_info::IconInfo, window_info::WindowInfo},
};
use error::xwin_error;
use napi::{bindgen_prelude::AsyncTask, JsFunction, JsNumber, Result, Task};
use napi_derive::napi;
use x_win::{empty_entity, get_active_window, get_browser_url, get_open_windows, get_window_icon};

#[macro_use]
extern crate napi_derive;

use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use std::{thread, time::Duration};

use once_cell::sync::Lazy;
use std::sync::Mutex;

static THREAD_MANAGER: Lazy<Mutex<ThreadManager>> = Lazy::new(|| Mutex::new(ThreadManager::new()));

pub struct OpenWindowsTask;
pub struct ActiveWindowTask;
pub struct GetIconTask {
  data: WindowInfo,
}

impl GetIconTask {
  pub fn new(data: WindowInfo) -> Self {
    Self { data }
  }
}

#[napi]
impl Task for OpenWindowsTask {
  type Output = Vec<WindowInfo>;
  type JsValue = Vec<WindowInfo>;

  fn compute(&mut self) -> Result<Self::Output> {
    open_windows()
  }

  fn resolve(&mut self, _: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
impl Task for ActiveWindowTask {
  type Output = WindowInfo;
  type JsValue = WindowInfo;

  fn compute(&mut self) -> Result<Self::Output> {
    active_window()
  }

  fn resolve(&mut self, _: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
impl Task for GetIconTask {
  type Output = IconInfo;
  type JsValue = IconInfo;

  fn compute(&mut self) -> Result<Self::Output> {
    get_icon(&self.data)
  }

  fn resolve(&mut self, _: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

fn get_icon(window_info: &WindowInfo) -> Result<IconInfo> {
  let t: x_win::WindowInfo = window_info.clone().into();
  match get_window_icon(&t) {
    Ok(window_icon) => Ok(window_icon.into()),
    Err(err) => Err(xwin_error(err)),
  }
}

fn get_url(window_info: &WindowInfo) -> Result<String> {
  let t: x_win::WindowInfo = window_info.clone().into();
  match get_browser_url(&t) {
    Ok(browser_url) => Ok(browser_url),
    Err(err) => Err(xwin_error(err)),
  }
}

#[napi]
impl WindowInfo {
  /**
   * Funciton who help to recover icon of application and will return `IconInfo`.
   */
  #[napi]
  pub fn get_icon(&self) -> Result<IconInfo> {
    get_icon(self)
  }

  /**
   * Promise funciton who help to recover icon of application and will return `IconInfo`.
   */
  #[napi]
  pub fn get_icon_async(&self) -> AsyncTask<GetIconTask> {
    let data = self;
    AsyncTask::new(GetIconTask { data: data.clone() })
  }

  /**
   * Getter to recover browser url
   */
  #[napi(getter)]
  pub fn url(&self) -> Result<String> {
    get_url(self)
  }
}

/**
 * Retrieve information the about currently active window.
 * Returns an object of `WindowInfo`.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { activeWindow } = require('@miniben90/x-win');
 *
 * const currentWindow = activeWindow();
 * console.log(currentWindow);
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { activeWindow } from '@miniben90/x-win';
 *
 * const currentWindow = activeWindow();
 * console.log(currentWindow);
 * ```
 *
 * # Information about Electron
 *
 * It is recommended to use this function within a worker to mitigate potential recovery issues on MacOS.
 */
#[napi]
pub fn active_window() -> Result<WindowInfo> {
  match get_active_window() {
    Ok(active_window) => Ok(active_window.into()),
    Err(err) => Err(xwin_error(err)),
  }
}

/**
 * Retrieve information about the currently active window as a promise.
 * Returns an object of `WindowInfo`.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * activeWindowAsync()
 * .then(currentWindow => {
 *   console.log(currentWindow);
 * });
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { activeWindowAsync } from '@miniben90/x-win';
 *
 * activeWindowAsync()
 * .then(currentWindow => {
 *   console.log(currentWindow);
 * });
 * ```
 *
 * # Information about Electron
 *
 * It is recommended to use this function within a worker to mitigate potential recovery issues on MacOS.
 */
#[napi]
pub fn active_window_async() -> AsyncTask<ActiveWindowTask> {
  AsyncTask::new(ActiveWindowTask {})
}

/**
 * Retrieve information about the currently open windows.
 * Returns an array of `WindowInfo`, each containing details about a specific open window.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { openWindows } = require('@miniben90/x-win');
 *
 * const windows = openWindows();
 * for (let i = 0; i < windows.length; i++) {
 *   console.log(i, windows[i]);
 * }
 * ```
 *
 * ## Typescript Example
 *
 * ```typescript
 * import { openWindows } from '@miniben90/x-win';
 *
 * const windows = openWindows();
 * for (let i = 0; i < windows.length; i++) {
 *   console.log(i, windows[i]);
 * }
 * ```
 *
 * # Information about Electron
 *
 * It is recommended to use this function within a worker to mitigate potential recovery issues on MacOS.
 */
#[napi]
pub fn open_windows() -> Result<Vec<WindowInfo>> {
  match get_open_windows() {
    Ok(open_windows) => Ok(open_windows.into_iter().map(WindowInfo::from).collect()),
    Err(err) => Err(xwin_error(err)),
  }
}

/**
 * Retrieve information about the currently open windows as a promise.
 * Returns an array of `WindowInfo`, each containing details about a specific open window.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { openWindowsAsync } = resuire('@miniben90/x-win');
 *
 * openWindowsAsync()
 * .then(windows => {
 *   for (let i = 0; i < windows.length; i++) {
 *     console.log(i, windows[i]);
 *   }
 * });
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { openWindowsAsync } from '@miniben90/x-win';
 *
 * openWindowsAsync()
 * .then(windows => {
 *   for (let i = 0; i < windows.length; i++) {
 *     console.log(i, windows[i]);
 *   }
 * });
 * ```
 *
 * # Information about Electron
 *
 * It is recommended to use this function within a worker to mitigate potential recovery issues on MacOS.
 */
#[napi]
pub fn open_windows_async() -> AsyncTask<OpenWindowsTask> {
  AsyncTask::new(OpenWindowsTask {})
}

/**
 * Subscribe an observer thread to monitor changes in the active window.
 * @param {function} callback - Callback function that returns the active window when it changes
 * @param {number} [interval=100] - Interval between checks for changes in the active window (default: 100ms)
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { subscribeActiveWindow, unsubscribeAllActiveWindow } = require('@miniben90/x-win');
 *
 * const a = subscribeActiveWindow((err, info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((err, info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((err, info) => {
 *   t.log(c, info);
 * });
 * const d = subscribeActiveWindow((err, info) => {
 *   t.log(c, info);
 * },500);// sleep interval: 500ms
 *
 * unsubscribeAllActiveWindow();
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { subscribeActiveWindow, unsubscribeAllActiveWindow } from '@miniben90/x-win';
 *
 * const a = subscribeActiveWindow((err, info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((err, info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((err, info) => {
 *   t.log(c, info);
 * });
 * const d = subscribeActiveWindow((err, info) => {
 *   t.log(c, info);
 * },500);// sleep interval: 500ms
 *
 * unsubscribeAllActiveWindow();
 * ```
 *
 */
#[napi(
  ts_args_type = "callback: (error: Error | null, info: WindowInfo | undefined) => void, interval?: number"
)]
pub fn subscribe_active_window(callback: JsFunction, interval: Option<JsNumber>) -> Result<u32> {
  let interval: u64 = {
    let interval = interval
      .map(|jsnumber| jsnumber.get_int64())
      .transpose()?
      .unwrap_or(100);
    if interval.gt(&0) {
      interval as u64
    } else {
      100
    }
  };
  let tsfn: ThreadsafeFunction<WindowInfo, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(
      0,
      |ctx: napi::threadsafe_function::ThreadSafeCallContext<WindowInfo>| Ok(vec![ctx.value]),
    )?;

  let tsfn_clone: ThreadsafeFunction<WindowInfo, ErrorStrategy::CalleeHandled> = tsfn.clone();

  let thread_manager = THREAD_MANAGER.lock().unwrap();

  let id = thread_manager.start_thread(move |receiver| {
    let mut current_window: WindowInfo = empty_entity().into();
    loop {
      match receiver.try_recv() {
        Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => {
          break;
        }
        _ => {
          match get_active_window() {
            Ok(new_current_window) => {
              if new_current_window.id.ne(&current_window.id)
                || new_current_window.title.ne(&current_window.title)
                || new_current_window
                  .info
                  .process_id
                  .ne(&current_window.info.process_id)
                || new_current_window.id.eq(&0)
              {
                current_window = new_current_window.clone().into();
                tsfn_clone.call(
                  Ok(new_current_window.into()),
                  ThreadsafeFunctionCallMode::Blocking,
                );
              }
            }
            Err(err) => {
              tsfn_clone.call(Err(xwin_error(err)), ThreadsafeFunctionCallMode::Blocking);
              break;
            }
          }
          thread::sleep(Duration::from_millis(interval));
        }
      }
    }
  });

  Ok(id.unwrap())
}

/**
 * Terminate and unsubscribe a specific observer using their ID.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { subscribeActiveWindow, unsubscribeActiveWindow } = require('@miniben90/x-win');
 *
 * const a = subscribeActiveWindow((info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((info) => {
 *   t.log(c, info);
 * });
 *
 * unsubscribeActiveWindow(a);
 * unsubscribeActiveWindow(b);
 * unsubscribeActiveWindow(c);
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { subscribeActiveWindow, unsubscribeActiveWindow } from '@miniben90/x-win';
 *
 * const a = subscribeActiveWindow((info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((info) => {
 *   t.log(c, info);
 * });
 *
 * unsubscribeActiveWindow(a);
 * unsubscribeActiveWindow(b);
 * unsubscribeActiveWindow(c);
 * ```
 */
#[napi]
pub fn unsubscribe_active_window(thread_id: u32) -> Result<()> {
  THREAD_MANAGER
    .lock()
    .unwrap()
    .stop_thread(thread_id)
    .unwrap();
  Ok(())
}

/**
 * Terminate and unsubscribe all observer threads monitoring changes in the active window.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { subscribeActiveWindow, unsubscribeAllActiveWindow } = require('@miniben90/x-win');
 *
 * const a = subscribeActiveWindow((info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((info) => {
 *   t.log(c, info);
 * });
 *
 * unsubscribeAllActiveWindow();
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { subscribeActiveWindow, unsubscribeAllActiveWindow } from '@miniben90/x-win';
 *
 * const a = subscribeActiveWindow((info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((info) => {
 *   t.log(c, info);
 * });
 *
 * unsubscribeAllActiveWindow();
 * ```
 */
#[napi]
pub fn unsubscribe_all_active_window() -> Result<()> {
  match THREAD_MANAGER.lock() {
    Ok(thread_manager) => match thread_manager.stop_all_threads() {
      Ok(_) => Ok(()),
      Err(_) => Ok(()),
    },
    Err(_) => Ok(()),
  }
}

/**
 * Install "@mininben90/x-win" Gnome extension required for Linux using Gnome > 41.
 * This function will write extension files needed to correctly detect working windows with Wayland desktop environment.
 * **Restart session will be require to install the gnome extension.**
 */
#[napi]
pub fn install_extension() -> Result<bool> {
  x_win::install_extension().map_err(xwin_error)
}

/**
 * Uninstall "@mininben90/x-win" Gnome extension.
 * This function will disable and remove extension files.
 * **Restart session will be require to remove the gnome extension.**
 */
#[napi]
pub fn uninstall_extension() -> Result<bool> {
  x_win::uninstall_extension().map_err(xwin_error)
}

/**
 * Enable Gnome extensions required for Linux using Gnome > 41.
 * This function will enable extension needed to correctly detect working windows with Wayland desktop environment.
 */
#[napi]
pub fn enable_extension() -> Result<bool> {
  x_win::enable_extension().map_err(xwin_error)
}

/**
 * Disable Gnome extensions required for Linux using Gnome > 41.
 * This function will disable extension needed to correctly detect working windows with Wayland desktop environment.
 */
#[napi]
pub fn disable_extension() -> Result<bool> {
  x_win::disable_extension().map_err(xwin_error)
}

/**
 * Return true of false if gnome extension is enabled for Linux using Gnome > 41.
 * This function will return true or false if the extension is set to enabled on extension info. Working only with Wayland windows manager.
 */
#[napi]
pub fn is_enabled_extension() -> Result<bool> {
  x_win::is_enabled_extension().map_err(xwin_error)
}

/**
 * Return true of false the extensions is installed for Linux using Gnome > 41.
 * This function will return true or false if the extension is correctly installed. Working only with Wayland windows manager.
 */
#[napi]
pub fn is_installed_extension() -> Result<bool> {
  x_win::is_installed_extension().map_err(xwin_error)
}
