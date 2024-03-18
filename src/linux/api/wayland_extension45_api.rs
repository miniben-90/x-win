use zbus::Connection;

use std::{env, fs, io, path};

use crate::{
  common::x_win_struct::window_info::WindowInfo,
  linux::api::gnome_shell::{
    value_to_window_info, GNOME_XWIN_EXTENSION_FOLDER_PATH, GNOME_XWIN_EXTENSION_META, GNOME_XWIN_UUID
  },
};

use super::{common_api::init_entity, gnome_shell::GNOME45_XWIN_EXTENSION_SCRIPT};

pub fn get_active_window() -> WindowInfo {
  let response = call_script(&"get_active_window".to_string());

  if response.ne(&"") {
    let response: serde_json::Value = serde_json::from_str(&response.as_str()).unwrap();
    if response.is_object() {
      return value_to_window_info(&response);
    }
  }

  init_entity()
}

pub fn get_open_windows() -> Vec<WindowInfo> {
  let response = call_script(&"get_open_windows".to_string());
  if response.ne(&"") {
    let response: serde_json::Value = serde_json::from_str(&response.as_str()).unwrap();

    if response.is_array() {
      return response
        .as_array()
        .unwrap()
        .iter()
        .map(|val| value_to_window_info(&val))
        .collect();
    }
  }

  vec![]
}

pub fn install_extension() -> () {
  if std::fs::metadata(get_extension_path()).is_ok() {
    if std::fs::metadata(get_extension_file_path()).is_ok() {
      let _t = remove_extension_file().unwrap();
    }
    if std::fs::metadata(get_medata_file_path()).is_ok() {
      let _t = remove_metadata_file().unwrap();
    }
  } else {
    if let Err(create_folder_err) = std::fs::create_dir(get_extension_path()) {
      panic!(
        "Not possible to create extension folder to \"{}\"!\nRaison: {}",
        get_extension_path().to_string_lossy(),
        create_folder_err.to_string()
      );
    }
  }
  if fs::write(get_extension_file_path(), GNOME45_XWIN_EXTENSION_SCRIPT.to_string()).is_ok() {
    if fs::write(get_medata_file_path(), GNOME_XWIN_EXTENSION_META).is_ok() {
      ()
    } else {
      let _t = remove_extension_file().unwrap();
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

pub fn uninstall_extension() -> () {
  if let Ok(_folder_dir) = cleanup_extension_folder() {
    let connection = Connection::new_session().unwrap();
    let _response = connection
      .call_method(
        Some("org.gnome.Shell"),
        "/org/gnome/Shell",
        Some("org.gnome.Shell.Extensions"),
        "DisableExtension",
        &GNOME_XWIN_UUID.to_string(),
      )
      .unwrap();
    let _t = cleanup_extension_folder();
    ()
  }
  ()
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

fn call_script(method_name: &String) -> String {
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
  "".to_owned()
}