use zbus::{blocking::Connection, Message};

use std::{env, fs, ops::Deref, path};

use crate::{
  common::{
    api::empty_icon,
    result::Result,
    x_win_struct::{icon_info::IconInfo, window_info::WindowInfo},
  },
  linux::api::gnome_shell::{
    value_to_window_info, GNOME45_XWIN_EXTENSION_SCRIPT, GNOME_SINGLETON,
    GNOME_XWIN_EXTENSION_COMMON_SCRIPT, GNOME_XWIN_EXTENSION_FOLDER_PATH,
    GNOME_XWIN_EXTENSION_META, GNOME_XWIN_EXTENSION_SCRIPT, GNOME_XWIN_UUID,
  },
};

use super::{
  common_api::init_entity,
  gnome_shell::{
    value_to_icon_info, DESTINATION, GNOME_XWIN_GET_ICON_SCRIPT, SHELL_IFACE, SHELL_PATH,
    XWIN_IFACE, XWIN_PATH,
  },
};

pub fn get_active_window() -> Result<WindowInfo> {
  let response = call_script("get_active_window");

  if response.is_err() {
    return Err(
      format!(
        r#"Unable to get informations of active window from "{}" extension via GNOME Shell. Please verify that the extension is correctly installed or enabled."#,
        GNOME_XWIN_UUID
      ).into()
    );
  }

  let response = response.unwrap();

  if !response.is_empty() {
    let response: serde_json::Value = serde_json::from_str(response.as_str())?;
    match response.is_object() {
      true => Ok(value_to_window_info(&response)),
      false => Err(String::from("No data founded for active window").into()),
    }
  } else {
    Ok(init_entity())
  }
}

pub fn get_open_windows() -> Result<Vec<WindowInfo>> {
  let response = call_script("get_open_windows");

  if response.is_err() {
    return Err(
      format!(
        r#"Unable to get informations of open windows from "{}" extension via GNOME Shell. Please verify that the extension is correctly installed or enabled."#,
        GNOME_XWIN_UUID
      ).into()
    );
  }

  let response = response.unwrap();

  if !response.is_empty() {
    let response: serde_json::Value = serde_json::from_str(response.as_str())?;

    let response = match response.as_array() {
      Some(result) => result.iter().map(value_to_window_info).collect(),
      None => vec![],
    };
    Ok(response)
  } else {
    Ok(vec![])
  }
}

pub fn get_icon(window_info: &WindowInfo) -> Result<IconInfo> {
  if window_info.id.ne(&0) {
    let response = call_script_arg("get_icon", window_info.id)?;
    if !response.is_empty() {
      let response: serde_json::Value = serde_json::from_str(response.as_str())?;
      if response.is_object() {
        return Ok(value_to_icon_info(&response));
      }
    }
  }
  Ok(empty_icon())
}

pub fn install_extension() -> Result<bool> {
  if std::fs::metadata(get_extension_path()).is_ok() {
    if std::fs::metadata(get_extension_file_path()).is_ok() {
      remove_extension_file()?;
    }
    if std::fs::metadata(get_medata_file_path()).is_ok() {
      remove_metadata_file()?;
    }
  } else if let Err(create_folder_err) = std::fs::create_dir_all(get_extension_path()) {
    panic!(
      "Not possible to create extension folder to \"{}\"!\nRaison: {}",
      get_extension_path().to_string_lossy(),
      create_folder_err
    );
  }

  let script: String = {
    let gnome_singleton = GNOME_SINGLETON.lock()?;
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
      Ok(true)
    } else {
      remove_extension_file()?;
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

fn toggle_extension(enable: bool) -> Result<bool> {
  let connection = Connection::session()?;
  let method_name = {
    if enable {
      "EnableExtension"
    } else {
      "DisableExtension"
    }
  };
  let response = connection.call_method(
    DESTINATION,
    SHELL_PATH,
    SHELL_IFACE,
    method_name,
    &GNOME_XWIN_UUID.to_string(),
  )?;

  if !response.body().is_empty() {
    let response: bool = response.body().deserialize()?;
    return Ok(response);
  }

  Err(format!(
        "Unable to enable or disable the {} extension via GNOME Shell. Please verify that the extension is correctly installed.",
        GNOME_XWIN_UUID
    ).into())
}

pub fn enable_extension() -> Result<bool> {
  toggle_extension(true)
}

pub fn disable_extension() -> Result<bool> {
  toggle_extension(false)
}

pub fn uninstall_extension() -> Result<bool> {
  if let Ok(_folder_dir) = cleanup_extension_folder() {
    cleanup_extension_folder()?;
    Ok(true)
  } else {
    Ok(false)
  }
}

pub fn is_enabled_extension() -> Result<bool> {
  let response = request_extension_info()?;
  let response: String = response.body().deserialize().unwrap_or_default();
  if !response.is_empty() {
    let response: serde_json::Value = serde_json::from_str(response.as_str())?;
    if response.is_object() {
      let response = response.as_object().unwrap();
      if response.contains_key("enabled") {
        let response = response["enabled"].as_bool().unwrap_or(false);
        return Ok(response);
      }
    }
  }

  Err(
    format!(
      r#"Unable to get information for "{}" extension via GNOME Shell. Please verify that the extension is correctly installed."#,
      GNOME_XWIN_UUID
    ).into()
  )
}

pub fn is_installed_extension() -> Result<bool> {
  let response = request_extension_info();
  if response.is_ok() {
    let response = response.unwrap();
    let response: String = response.body().deserialize().unwrap_or_default();
    if !response.is_empty() {
      return Ok(true);
    }
  }
  Ok(false)
}

fn request_extension_info() -> Result<Message> {
  let connection = Connection::session()?;
  Ok(connection.call_method(
    DESTINATION,
    SHELL_PATH,
    SHELL_IFACE,
    "GetExtensionInfo",
    &GNOME_XWIN_UUID.to_string(),
  )?)
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

fn cleanup_extension_folder() -> Result<()> {
  Ok(fs::remove_dir_all(get_extension_path())?)
}

fn get_extension_file_path() -> path::PathBuf {
  get_extension_path().join("extension.js")
}

fn get_medata_file_path() -> path::PathBuf {
  get_extension_path().join("metadata.json")
}

fn remove_metadata_file() -> Result<()> {
  Ok(fs::remove_file(get_medata_file_path())?)
}

fn remove_extension_file() -> Result<()> {
  Ok(fs::remove_file(get_extension_file_path())?)
}

fn call_script(method_name: &str) -> Result<String> {
  let connection = Connection::session()?;

  let response = connection.call_method(DESTINATION, XWIN_PATH, XWIN_IFACE, method_name, &())?;

  if !response.body().is_empty() {
    let response: String = response.body().deserialize()?;
    return Ok(response);
  }

  Err(
    String::from(
      "No result when calling org.gnome.Shell.Extensions.XWinWaylandExtension gnome script!",
    )
    .into(),
  )
}

fn call_script_arg(method_name: &str, body: u32) -> Result<String> {
  let connection = Connection::session()?;

  let response = connection.call_method(
    DESTINATION,
    XWIN_PATH,
    XWIN_IFACE,
    method_name,
    &(body as f64),
  )?;

  if !response.body().is_empty() {
    let response: String = response.body().deserialize()?;
    return Ok(response);
  }

  Err(
    String::from(
      "No result when calling org.gnome.Shell.Extensions.XWinWaylandExtension gnome script!",
    )
    .into(),
  )
}
