use zbus::Connection;

use crate::{
  common::x_win_struct::{icon_info::IconInfo, window_info::WindowInfo},
  linux::api::gnome_shell::GNOME_XWIN_EVAL_SCRIPT,
};

use super::{
  common_api::init_entity,
  gnome_shell::{value_to_icon_info, value_to_window_info, GNOME_XWIN_GET_ICON_SCRIPT},
};

pub fn get_active_window() -> WindowInfo {
  let script = format!(
    r#"
{}
get_active_window();
"#,
    GNOME_XWIN_EVAL_SCRIPT
  );

  let response = call_script(&script);

  if response.ne(&"") {
    let response: serde_json::Value = serde_json::from_str(response.as_str()).unwrap();
    if response.is_object() {
      return value_to_window_info(&response);
    }
  }

  init_entity()
}

pub fn get_open_windows() -> Vec<WindowInfo> {
  let script = format!(
    r#"
{}

get_open_windows();
"#,
    GNOME_XWIN_EVAL_SCRIPT
  );

  let response = call_script(&script);
  if !response.is_empty() {
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

fn call_script(script: &String) -> String {
  let connection = Connection::new_session().unwrap();

  let response = connection
    .call_method(
      Some("org.gnome.Shell"),
      "/org/gnome/Shell",
      Some("org.gnome.Shell"),
      "Eval",
      script,
    )
    .unwrap();

  if let Ok((_actor, json)) = response.body::<(bool, String)>() {
    return json;
  }
  String::from("")
}

pub fn get_icon(window_info: &WindowInfo) -> IconInfo {
  if window_info.id.ne(&0) {
    let script = format!(
      r#"
{}
{}
get_icon({});
"#,
      GNOME_XWIN_EVAL_SCRIPT, GNOME_XWIN_GET_ICON_SCRIPT, window_info.id
    );

    let response = call_script(&script);

    if !response.is_empty() {
      let response: serde_json::Value = serde_json::from_str(response.as_str()).unwrap();
      if response.is_object() {
        return value_to_icon_info(&response);
      }
    }
  }

  IconInfo {
    data: String::from(""),
    height: 0,
    width: 0,
  }
}
