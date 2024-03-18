use zbus::Connection;

use crate::{
  common::x_win_struct::window_info::WindowInfo,
  linux::api::gnome_shell::GNOME_XWIN_COMMON_FN,
};

use super::{common_api::init_entity, gnome_shell::value_to_window_info};

pub fn get_active_window() -> WindowInfo {
  let script = format!(
    r#"
{}

get_active_window();
"#,
    GNOME_XWIN_COMMON_FN
  );

  let response = call_script(&script);

  if response.ne(&"") {
    let response: serde_json::Value = serde_json::from_str(&response.as_str()).unwrap();
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
    GNOME_XWIN_COMMON_FN
  );

  let response = call_script(&script);
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
  "".to_owned()
}
