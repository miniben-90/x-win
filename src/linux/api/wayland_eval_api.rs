use zbus::Connection;

use crate::{
  common::x_win_struct::{
    process_info::ProcessInfo, usage_info::UsageInfo, window_info::WindowInfo,
    window_position::WindowPosition,
  },
  linux::api::gnome_shell::GNOME_XWIN_COMMON_FN,
};

use super::common_api::init_entity;

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

fn number_to_u32(value: &serde_json::Value) -> u32 {
  if value.is_number() {
    return value.as_u64().unwrap() as u32;
  }
  0
}

fn number_to_i32(value: &serde_json::Value) -> i32 {
  if value.is_number() {
    return value.as_i64().unwrap() as i32;
  }
  0
}

fn value_to_window_info(response: &serde_json::Value) -> WindowInfo {
  let response = response.as_object().unwrap();
  let position = response["position"].as_object().unwrap();
  let info = response["info"].as_object().unwrap();
  let usage = response["usage"].as_object().unwrap();
  WindowInfo {
    id: number_to_u32(&response["id"]),
    os: response["os"].as_str().unwrap().to_string(),
    title: response["title"].as_str().unwrap().to_string(),
    position: WindowPosition {
      height: number_to_i32(&position["height"]),
      width: number_to_i32(&position["width"]),
      x: number_to_i32(&position["x"]),
      y: number_to_i32(&position["y"]),
    },
    info: ProcessInfo {
      exec_name: info["exec_name"].as_str().unwrap().to_string(),
      name: info["name"].as_str().unwrap().to_string(),
      path: info["path"].as_str().unwrap().to_string(),
      process_id: number_to_u32(&info["process_id"]),
    },
    usage: UsageInfo {
      memory: number_to_u32(&usage["memory"]),
    },
    url: "".to_owned(),
  }
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
