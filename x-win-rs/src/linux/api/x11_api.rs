#![deny(unused_imports)]

use base64::Engine;
use image::ImageFormat;

use xcb::{x, Connection, Xid, XidNew};

use crate::{
  common::{
    api::Api,
    x_win_struct::{icon_info::IconInfo, window_info::WindowInfo, window_position::WindowPosition},
  },
  linux::api::common_api::{get_window_memory_usage, get_window_path_name},
};

use super::common_api::init_entity;

/**
 * Struct to use similar as API to get active window and open windows for XOrg desktop
 */
pub struct X11Api {}

/**
 * Impl. for windows system
 */
impl Api for X11Api {
  fn get_active_window(&self) -> WindowInfo {
    let conn = connection();
    let setup = conn.get_setup();

    let mut result: WindowInfo = init_entity();

    let root_window = setup.roots().next();
    if root_window.is_some() {
      let root_window = root_window.unwrap().root();
      let active_window_atom = get_active_window_atom(&conn);
      if active_window_atom != x::ATOM_NONE {
        let active_window = conn.send_request(&x::GetProperty {
          delete: false,
          window: root_window,
          property: active_window_atom,
          r#type: x::ATOM_WINDOW,
          long_offset: 0,
          long_length: 1,
        });
        if let Ok(active_window) = conn.wait_for_reply(active_window) {
          let active_window: Option<&x::Window> = active_window.value::<x::Window>().first();
          if active_window.is_some() {
            let active_window: &x::Window = active_window.unwrap();
            result = get_window_information(&conn, active_window);
          }
        }
      }
    }

    result
  }

  fn get_open_windows(&self) -> Vec<WindowInfo> {
    let mut results: Vec<WindowInfo> = Vec::new();

    let conn = connection();
    let setup = conn.get_setup();

    let root_window = setup.roots().next();
    if root_window.is_some() {
      let root_window = root_window.unwrap().root();

      let open_windows_atom = get_client_list_stacking_atom(&conn);
      if open_windows_atom != x::ATOM_NONE {
        let window_list = conn.send_request(&x::GetProperty {
          delete: false,
          window: root_window,
          property: open_windows_atom,
          r#type: x::ATOM_WINDOW,
          long_offset: 0,
          long_length: u32::MAX,
        });
        if let Ok(windows_reply) = conn.wait_for_reply(window_list) {
          let window_list: Vec<x::Window> = windows_reply.value::<x::Window>().to_vec();
          if window_list.len().ne(&0) {
            for window in window_list {
              let window: &x::Window = &window;
              let result = get_window_information(&conn, window);
              if result.id.ne(&0) && is_normal_window(&conn, *window) {
                results.push(result);
              }
            }
          }
        }
      }
    }
    results
  }

  fn get_app_icon(&self, window_info: &WindowInfo) -> IconInfo {
    let conn = connection();
    let setup = conn.get_setup();

    let root_window = setup.roots().next();
    if root_window.is_some() {
      let window = unsafe { XidNew::new(window_info.id) };
      let icon_atom = get_window_icon_atom(&conn);
      if icon_atom != x::ATOM_NONE {
        let icon_cookie = conn.send_request(&x::GetProperty {
          delete: false,
          window,
          property: icon_atom,
          r#type: x::ATOM_CARDINAL,
          long_offset: 0,
          long_length: u32::MAX,
        });
        if let Ok(icon_reply) = conn.wait_for_reply(icon_cookie) {
          let icon_data: &[u32] = icon_reply.value::<u32>();
          if !icon_data.is_empty() {
            let width = icon_data[0] as usize;
            let height = icon_data[1] as usize;
            let bgra_data: &[u32] = &icon_data[2..2 + (width * height)];

            let mut buffer: Vec<u8> = vec![0u8; height * width * 4];

            for (i, &bgra) in bgra_data.iter().enumerate() {
              let b = (bgra & 0xFF) as u8;
              let g = ((bgra >> 8) & 0xFF) as u8;
              let r = ((bgra >> 16) & 0xFF) as u8;
              let a = ((bgra >> 24) & 0xFF) as u8;
              let index = i * 4;
              buffer[index] = r;
              buffer[index + 1] = g;
              buffer[index + 2] = b;
              buffer[index + 3] = a;
            }
            let mut png_data: Vec<u8> = Vec::new();
            {
              let buffer = image::RgbaImage::from_raw(width as u32, height as u32, buffer).unwrap();
              let _ = buffer.write_to(&mut std::io::Cursor::new(&mut png_data), ImageFormat::Png);
              let data = base64::prelude::BASE64_STANDARD.encode(png_data);
              return IconInfo {
                data: format!("data:image/png;base64,{}", data).to_owned(),
                height: height as u32,
                width: width as u32,
              };
            }
          }
        }
      }
    }
    IconInfo {
      data: "".to_owned(),
      width: 0,
      height: 0,
    }
  }

  fn get_browser_url(&self, _: &WindowInfo) -> String {
    "URL recovery not supported on Linux distribution!".to_owned()
  }
}

fn connection() -> Connection {
  let (conn, _) = xcb::Connection::connect(None).unwrap();
  conn
}

/**
 * Get window information
 */
fn get_window_information(conn: &xcb::Connection, window: &x::Window) -> WindowInfo {
  let window_pid: u32 = get_window_pid(conn, *window);
  let mut window_info: WindowInfo = init_entity();

  if window_pid != 0 {
    let (path, exec_name) = get_window_path_name(window_pid);
    window_info.id = window.resource_id();
    window_info.title = get_window_title(conn, *window);
    window_info.info.process_id = window_pid;
    window_info.info.path = path;
    window_info.info.exec_name = exec_name;
    window_info.info.name = get_window_class_name(conn, *window);
    window_info.usage.memory = get_window_memory_usage(window_pid);
    window_info.position = get_window_position(conn, *window);
  }
  window_info
}

/**
 * Get pid
 */
fn get_window_pid(conn: &xcb::Connection, window: x::Window) -> u32 {
  let window_pid_atom = get_window_pid_atom(conn);
  if window_pid_atom != x::ATOM_NONE {
    let window_pid = conn.send_request(&x::GetProperty {
      delete: false,
      window,
      property: window_pid_atom,
      r#type: x::ATOM_ANY,
      long_offset: 0,
      long_length: 1,
    });
    if let Ok(window_pid) = conn.wait_for_reply(window_pid) {
      return window_pid.value::<u32>().first().unwrap().to_owned();
    }
  }
  0
}

/**
 * Get window width, height, x and y
 */
fn get_window_position(conn: &xcb::Connection, window: x::Window) -> WindowPosition {
  let mut position = WindowPosition {
    x: 0,
    y: 0,
    width: 0,
    height: 0,
    is_full_screen: is_full_screen_window(conn, window),
  };
  let window_geometry = conn.send_request(&x::GetGeometry {
    drawable: x::Drawable::Window(window),
  });
  if let Ok(window_geometry) = conn.wait_for_reply(window_geometry) {
    position.height = window_geometry.height() as i32;
    position.width = window_geometry.width() as i32;
    let window_geometry_x = window_geometry.x();
    let window_geometry_y = window_geometry.y();
    let translated_position = conn.send_request(&x::TranslateCoordinates {
      dst_window: window_geometry.root(),
      src_window: window,
      src_x: window_geometry_x,
      src_y: window_geometry_y,
    });
    if let Ok(translated_position) = conn.wait_for_reply(translated_position) {
      position.x = (translated_position.dst_x() - window_geometry_x) as i32;
      position.y = (translated_position.dst_y() - window_geometry_y) as i32;
    }
  }

  position
}

/**
 * Get window title
 */
fn get_window_title(conn: &xcb::Connection, window: x::Window) -> String {
  _get_string_response(conn, window, x::ATOM_WM_NAME)
}

fn _get_string_response(conn: &xcb::Connection, window: x::Window, property: x::Atom) -> String {
  let window_title = conn.send_request(&x::GetProperty {
    delete: false,
    window,
    property,
    r#type: x::ATOM_NONE,
    long_offset: 0,
    long_length: u32::MAX,
  });
  if let Ok(window_title) = conn.wait_for_reply(window_title) {
    let window_title: &[u8] = window_title.value();
    unsafe { std::str::from_utf8_unchecked(window_title).to_string() }
  } else {
    "".to_owned()
  }
}

/**
 * Get process name
 */
fn get_window_class_name(conn: &xcb::Connection, window: x::Window) -> String {
  let window_class = conn.send_request(&x::GetProperty {
    delete: false,
    window,
    property: x::ATOM_WM_CLASS,
    r#type: x::ATOM_STRING,
    long_offset: 0,
    long_length: u32::MAX,
  });
  if let Ok(window_class) = conn.wait_for_reply(window_class) {
    let window_class = window_class.value();
    let window_class = std::str::from_utf8(window_class);

    let mut process_name = window_class
      .unwrap_or("")
      .split('\u{0}')
      .filter(|str| !str.is_empty())
      .collect::<Vec<&str>>();
    return process_name.pop().unwrap_or("").to_owned();
  }
  "".into()
}

fn get_window_pid_atom(conn: &xcb::Connection) -> x::Atom {
  get_atom(conn, b"_NET_WM_PID", true)
}

/**
 * Generate Atom of _NET_ACTIVE_WINDOW value
 */
fn get_active_window_atom(conn: &xcb::Connection) -> x::Atom {
  get_atom(conn, b"_NET_ACTIVE_WINDOW", true)
}

/**
 * Generate Atom of _NET_CLIENT_LIST_STACKING value
 */
fn get_client_list_stacking_atom(conn: &xcb::Connection) -> x::Atom {
  get_atom(conn, b"_NET_CLIENT_LIST_STACKING", true)
}

/**
 * Generate Atom of _NET_WM_WINDOW_TYPE value
 */
fn get_window_type_atom(conn: &xcb::Connection) -> x::Atom {
  get_atom(conn, b"_NET_WM_WINDOW_TYPE", true)
}

/**
 * Generate Atom of _NET_WM_WINDOW_TYPE_NORMAL value
 */
fn get_window_type_normal_atom(conn: &xcb::Connection) -> x::Atom {
  get_atom(conn, b"_NET_WM_WINDOW_TYPE_NORMAL", true)
}

/**
 * Generate Atom of _NET_WM_STATE value
 */
fn get_window_state_atom(conn: &xcb::Connection) -> x::Atom {
  get_atom(conn, b"_NET_WM_STATE", false)
}

/**
 * Generate Atom of _NET_WM_STATE_FULLSCREEN value
 */
fn get_window_state_fullscreen_atom(conn: &xcb::Connection) -> x::Atom {
  get_atom(conn, b"_NET_WM_STATE_FULLSCREEN", false)
}

/**
 * Generate Atom of _NET_WM_ICON value
 */
fn get_window_icon_atom(conn: &xcb::Connection) -> x::Atom {
  get_atom(conn, b"_NET_WM_ICON", false)
}

/**
 * Generate Atom of name parameter
 */
fn get_atom(conn: &xcb::Connection, name: &[u8], only_if_exists: bool) -> x::Atom {
  let atom_name = conn.send_request(&x::InternAtom {
    only_if_exists,
    name,
  });
  if let Ok(value) = conn.wait_for_reply(atom_name) {
    value.atom()
  } else {
    x::ATOM_NONE
  }
}

/**
 * Check if the window is a normal type
 */
fn is_normal_window(conn: &xcb::Connection, window: x::Window) -> bool {
  let window_type_atom = get_window_type_atom(conn);
  let type_normal_atom = get_window_type_normal_atom(conn);
  if window_type_atom != x::ATOM_NONE && type_normal_atom != x::ATOM_NONE {
    let window_state = conn.send_request(&x::GetProperty {
      delete: false,
      window,
      property: window_type_atom,
      r#type: x::ATOM_ATOM,
      long_offset: 0,
      long_length: u32::MAX,
    });
    if let Ok(window_state) = conn.wait_for_reply(window_state) {
      return window_state.value().contains(&type_normal_atom);
    }
  }
  false
}

/**
 * Check if the window is full screened
 */
fn is_full_screen_window(conn: &xcb::Connection, window: x::Window) -> bool {
  let state_window_atom = get_window_state_atom(conn);
  let state_fullscreen_atom = get_window_state_fullscreen_atom(conn);
  if state_window_atom != x::ATOM_NONE && state_fullscreen_atom != x::ATOM_NONE {
    let window_state = conn.send_request(&x::GetProperty {
      delete: false,
      window,
      property: state_window_atom,
      r#type: x::ATOM_ATOM,
      long_offset: 0,
      long_length: u32::MAX,
    });
    if let Ok(window_state) = conn.wait_for_reply(window_state) {
      return window_state.value().contains(&state_fullscreen_atom);
    }
  }
  false
}
