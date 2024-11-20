use zbus::Connection;

use std::{env, fs, io, ops::Deref, path};

use crate::{
  common::x_win_struct::{icon_info::IconInfo, window_info::WindowInfo},
  linux::api::gnome_shell::{
    value_to_window_info, GNOME45_XWIN_EXTENSION_SCRIPT, GNOME_SINGLETON,
    GNOME_XWIN_EXTENSION_COMMON_SCRIPT, GNOME_XWIN_EXTENSION_FOLDER_PATH,
    GNOME_XWIN_EXTENSION_META, GNOME_XWIN_EXTENSION_SCRIPT, GNOME_XWIN_UUID,
  },
};

use super::{
  common_api::{empty_icon, init_entity},
  gnome_shell::{value_to_icon_info, GNOME_XWIN_GET_ICON_SCRIPT},
};

pub fn get_active_window() -> WindowInfo {
  let response = call_script("get_active_window");

  if response.ne(&"") {
    let response: serde_json::Value = serde_json::from_str(response.as_str()).unwrap();
    if response.is_object() {
      return value_to_window_info(&response);
    }
  }

  init_entity()
}

pub fn get_open_windows() -> Vec<WindowInfo> {
  let response = call_script("get_open_windows");
  if response.ne(&"") {
    let response: serde_json::Value = serde_json::from_str(response.as_str()).unwrap();

    if response.is_array() {
      return response
        .as_array()
        .unwrap()
        .iter()
        .map(value_to_window_info)
        .collect();
    }
  }

  vec![]
}

pub fn get_icon(window_info: &WindowInfo) -> IconInfo {
  if window_info.id.ne(&0) {
    let response = call_script_arg("get_icon", window_info.id);
    if response.ne(&"") {
      let response: serde_json::Value = serde_json::from_str(response.as_str()).unwrap();
      if response.is_object() {
        return value_to_icon_info(&response);
      }
    }
  }
  empty_icon()
}

pub fn install_extension() -> bool {
  if std::fs::metadata(get_extension_path()).is_ok() {
    if std::fs::metadata(get_extension_file_path()).is_ok() {
      remove_extension_file().unwrap();
    }
    if std::fs::metadata(get_medata_file_path()).is_ok() {
      remove_metadata_file().unwrap();
    }
  } else if let Err(create_folder_err) = std::fs::create_dir_all(get_extension_path()) {
    panic!(
      "Not possible to create extension folder to \"{}\"!\nRaison: {}",
      get_extension_path().to_string_lossy(),
      create_folder_err
    );
  }

  let script: String = {
    let gnome_singleton = GNOME_SINGLETON.lock().unwrap();
    let version: u32 = gnome_singleton.version;
    let _ = gnome_singleton.deref();
    let script: String = match version {
      x if x.lt(&45) => GNOME_XWIN_EXTENSION_SCRIPT.to_string(),
      _ => GNOME45_XWIN_EXTENSION_SCRIPT.to_string(),
    };

    let script: String = format!(
      r#"{}

{}

{}
"#,
      script, GNOME_XWIN_EXTENSION_COMMON_SCRIPT, GNOME_XWIN_GET_ICON_SCRIPT
    );

    script
  };

  if fs::write(get_extension_file_path(), script).is_ok() {
    if fs::write(get_medata_file_path(), GNOME_XWIN_EXTENSION_META).is_ok() {
      true
    } else {
      remove_extension_file().unwrap();
      panic!(
        "Not possible to write \"{}\" file!",
        get_medata_file_path().to_string_lossy()
      );
    }
  } else {
    panic!(
      "Not possible to write \"{}\" file!",
      get_extension_file_path().to_string_lossy()
    );
  }
}

pub fn enable_extension() -> bool {
  let connection = Connection::new_session().unwrap();
  let response = connection
    .call_method(
      Some("org.gnome.Shell"),
      "/org/gnome/Shell",
      Some("org.gnome.Shell.Extensions"),
      "EnableExtension",
      &GNOME_XWIN_UUID.to_string(),
    )
    .unwrap();
  if let Ok(actor) = response.body::<bool>() {
    return actor;
  }
  false
}

pub fn uninstall_extension() -> bool {
  if let Ok(_folder_dir) = cleanup_extension_folder() {
    let _t = cleanup_extension_folder();
    true
  } else {
    false
  }
}

pub fn disable_extension() -> bool {
  let connection = Connection::new_session().unwrap();
  let response = connection
    .call_method(
      Some("org.gnome.Shell"),
      "/org/gnome/Shell",
      Some("org.gnome.Shell.Extensions"),
      "DisableExtension",
      &GNOME_XWIN_UUID.to_string(),
    )
    .unwrap();
  if let Ok(actor) = response.body::<bool>() {
    return actor;
  }
  false
}

fn get_extension_path() -> path::PathBuf {
  let home_dir: String = env::var_os("HOME")
    .unwrap()
    .clone()
    .to_string_lossy()
    .to_string();
  let extension_dir: String = GNOME_XWIN_EXTENSION_FOLDER_PATH.to_owned();
  [home_dir, extension_dir].iter().collect()
}

fn cleanup_extension_folder() -> Result<(), io::Error> {
  fs::remove_dir_all(get_extension_path())
}

fn get_extension_file_path() -> path::PathBuf {
  get_extension_path().join("extension.js")
}

fn get_medata_file_path() -> path::PathBuf {
  get_extension_path().join("metadata.json")
}

fn remove_metadata_file() -> Result<(), io::Error> {
  fs::remove_file(get_medata_file_path())
}

fn remove_extension_file() -> Result<(), std::io::Error> {
  fs::remove_file(get_extension_file_path())
}

fn call_script(method_name: &str) -> String {
  let connection = Connection::new_session().unwrap();

  let response = connection
    .call_method(
      Some("org.gnome.Shell"),
      "/org/gnome/Shell/Extensions/XWinWaylandExtension",
      Some("org.gnome.Shell.Extensions.XWinWaylandExtension"),
      method_name,
      &(),
    )
    .unwrap();

  if let Ok(json) = response.body::<String>() {
    return json;
  }
  String::from("")
}

fn call_script_arg(method_name: &str, body: u32) -> String {
  let connection = Connection::new_session().unwrap();

  let response = connection
    .call_method(
      Some("org.gnome.Shell"),
      "/org/gnome/Shell/Extensions/XWinWaylandExtension",
      Some("org.gnome.Shell.Extensions.XWinWaylandExtension"),
      method_name,
      &(body as f64),
    )
    .unwrap();

  if let Ok(json) = response.body::<String>() {
    return json;
  }
  String::from("")
}
