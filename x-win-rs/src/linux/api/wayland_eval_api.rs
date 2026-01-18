use zbus::blocking::Connection;

use crate::{
  common::{
    api::empty_icon,
    result::Result,
    x_win_struct::{icon_info::IconInfo, window_info::WindowInfo},
  },
  linux::api::gnome_shell::GNOME_XWIN_EVAL_SCRIPT,
};

use super::{
  common_api::init_entity,
  gnome_shell::{
    value_to_icon_info, value_to_window_info, DESTINATION, GNOME_XWIN_GET_ICON_SCRIPT, SHELL_IFACE,
    SHELL_PATH,
  },
};

pub fn get_active_window() -> Result<WindowInfo> {
  let script = format!(
    r#"
{GNOME_XWIN_EVAL_SCRIPT}
get_active_window();
"#
  );

  let response = call_script(&script)?;

  if !response.is_empty() {
    let response: serde_json::Value = serde_json::from_str(response.as_str())?;
    if response.is_object() {
      return Ok(value_to_window_info(&response)?);
    }
  }

  Ok(init_entity())
}

pub fn get_open_windows() -> Result<Vec<WindowInfo>> {
  let script = format!(
    r#"
{GNOME_XWIN_EVAL_SCRIPT}

get_open_windows();
"#
  );

  let response = call_script(&script)?;
  if !response.is_empty() {
    let response: serde_json::Value = serde_json::from_str(response.as_str())?;
    match response.as_array() {
      Some(values) => {
        let mut response: Vec<WindowInfo> = vec![];
        for value in values {
          let value = value_to_window_info(value)?;
          response.push(value);
        }
        return Ok(response);
      }
      _ => return Ok(vec![]),
    }
  }
  Ok(vec![])
}

fn call_script(script: &String) -> Result<String> {
  let connection = Connection::session()?;

  let response = connection.call_method(DESTINATION, SHELL_PATH, SHELL_IFACE, "Eval", script)?;

  if !response.body().is_empty() {
    let response: String = response.body().deserialize()?;
    return Ok(response);
  }

  Err(String::from("Not possible to execute eval gnome shell").into())
}

pub fn get_icon(window_info: &WindowInfo) -> Result<IconInfo> {
  if window_info.id.ne(&0) {
    let script = format!(
      r#"
{GNOME_XWIN_EVAL_SCRIPT}
{GNOME_XWIN_GET_ICON_SCRIPT}
get_icon({0});
"#,
      window_info.id
    );

    let response = call_script(&script)?;

    if !response.is_empty() {
      let response: serde_json::Value = serde_json::from_str(response.as_str())?;
      if response.is_object() {
        return Ok(value_to_icon_info(&response)?);
      }
    }
  }

  Ok(empty_icon())
}
