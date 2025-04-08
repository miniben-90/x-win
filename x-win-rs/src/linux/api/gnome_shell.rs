use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::common::x_win_struct::{
  icon_info::IconInfo, process_info::ProcessInfo, usage_info::UsageInfo, window_info::WindowInfo,
  window_position::WindowPosition,
};

use super::common_api::get_gnome_version;

pub const GNOME_XWIN_UUID: &str = r#"x-win@miniben90.org"#;

pub static DESTINATION: std::option::Option<&str> = Some("org.gnome.Shell");
pub static XWIN_IFACE: std::option::Option<&str> =
  Some("org.gnome.Shell.Extensions.XWinWaylandExtension");
pub static XWIN_PATH: &str = "/org/gnome/Shell/Extensions/XWinWaylandExtension";
pub static SHELL_PATH: &str = "/org/gnome/Shell";
pub static SHELL_IFACE: std::option::Option<&str> = Some("org.gnome.Shell.Extensions");

// Metadata for gnome version > then 41
pub const GNOME_XWIN_EXTENSION_META: &str = r#"
{
  "name": "@mininben90/x-win extended",
  "description": "Get active and open window(s) informations for @miniben90/x-win node package.",
  "uuid": "x-win@miniben90.org",
  "shell-version": [ "42", "43", "44", "45", "46" ],
  "url": "https://github.com/miniben-90/x-win.git",
  "version-name": "1.8.0"
}
"#;

pub const GNOME_XWIN_EXTENSION_FOLDER_PATH: &str =
  r#".local/share/gnome-shell/extensions/x-win@miniben90.org"#;

pub const GNOME_XWIN_GET_ICON_SCRIPT: &str = r#"function _get_icon(window_id) {
  if (window_id) {
    let meta_window = global.get_window_actors()
      .filter(x => _filterWindow)
      .find(x => x && x.get_meta_window && x.get_meta_window() && x.get_meta_window().get_id() === window_id)
      .get_meta_window();
    if (meta_window) { 
      const tracker = Shell.WindowTracker.get_default();
      const window_app = tracker.get_window_app(meta_window);
      if (window_app) {
        const icon = window_app.get_icon();
        if (icon) {
          const iconTheme = new St.IconTheme();
          const iconInfo = iconTheme.lookup_by_gicon(icon, 128, St.IconLookupFlags.FORCE_SIZE);
          if (iconInfo) {
            const pixBuf = iconInfo.load_icon();
            if (pixBuf) {
              const [ success, unitArray ] = pixBuf.save_to_bufferv('png', [], []);
              if(success && unitArray.length) {
                const data = GLib.base64_encode(unitArray);
                if (data) {
                  return {
                    data: "data:image/png;base64," + data,
                    height: pixBuf.get_height(),
                    width: pixBuf.get_width(),
                  };
                }
              }
            }
          }
        }
      }
    }
  }
  return {
    data: "",
    height: 0,
    width: 0,
  };
}"#;

pub const GNOME_XWIN_EVAL_SCRIPT: &str = r#"
const { Gio, GLib, Meta, Gtk: St, Shell } = imports.gi;

const AllowedWindow = [
  Meta.WindowType.DESKTOP,
  Meta.WindowType.OVERRIDE_OTHER,
  Meta.WindowType.UTILITY,
  Meta.WindowType.MODAL_DIALOG,
  Meta.WindowType.DIALOG,
  Meta.WindowType.NORMAL
];

function _filterWindow(x, index, array) {
  if (x && x.get_meta_window && x.get_meta_window().get_window_type) {
    return x.get_meta_window().get_window_type() !== -1;
  } else {
    return false;
  }
}

function _get_open_windows() {
  return global.get_window_actors()
    .filter(_filterWindow)
    .map(_strcut_data);
}

function get_open_windows() {
  return Object(_get_open_windows());
}

function get_active_window() {
  const activeWindow = global.get_window_actors().find(x => x.get_meta_window().has_focus() && _filterWindow(x));
  return Object(_strcut_data(activeWindow));
}

function get_icon(window_id) {
  const iconInfo = _get_icon(window_id);
  return Object(iconInfo);
}

function _strcut_data(window_actor) {
  if (window_actor && window_actor.get_meta_window) {
    const _window = window_actor.get_meta_window();

    const process_id = _window.get_pid ? _window.get_pid() : 0;
    const info = _get_process_info(process_id);

    return {
      id: _window.get_id(),
      os: 'linux',
      info: {
        process_id,
        name: _window.get_wm_class ? _window.get_wm_class() : '',
        path: info.path,
        exec_name: info.exec_name,
      },
      title: _window.get_title ? _window.get_title() : '',
      position: {
        width: window_actor.get_width ? window_actor.get_width() : 0,
        height: window_actor.get_height ? window_actor.get_height() : 0,
        x: window_actor.get_x ? window_actor.get_x() : 0,
        y: window_actor.get_y ? window_actor.get_y() : 0,
        isFullScreen: _window.is_fullscreen(),
      },
      usage: { memory: _get_memory_usage(process_id) },
    };
  } else {
    return {
      id: 0,
      os: 'linux',
      title: '',
      info: {
        process_id: 0,
        path: '',
        exec_name: '',
      },
      position: {
        width: 0,
        height: 0,
        x: 0,
        y: 0,
        isFullScreen: false,
      },
      usage: { memory: 0 },
    };
  }
}

function _get_memory_usage(pid) {
  const [isOk, contents] = GLib.file_get_contents(`/proc/${pid}/statm`);
  if (isOk) {
    return parseInt(contents.toString().split(' ')[0], 10);
  }
  return 0;
}

function _get_process_info(pid) {
  try {
    const path = GLib.file_read_link(`/proc/${pid}/exe`, null);
    if (path) {
      return {
        path,
        exec_name: path.split('/').pop(),
      };
    }
  } catch (e) { }
  return {
    path: '',
    exec_name: '',
  };
}"#;

pub const GNOME_XWIN_EXTENSION_COMMON_SCRIPT: &str = r#"
const WaylandInterface = `
<node>
  <interface name="org.gnome.Shell.Extensions.XWinWaylandExtension">
    <method name="get_active_window">
      <arg name="value" type="s" direction="out" />
    </method>
    <method name="get_open_windows">
      <arg name="value" type="s" direction="out" />
    </method>
    <method name="get_icon">
      <arg name="value" type="d" direction="in" />
      <arg name="value" type="s" direction="out" />
    </method>
  </interface>
</node>
`;

const AllowedWindow = [
  Meta.WindowType.DESKTOP,
  Meta.WindowType.OVERRIDE_OTHER,
  Meta.WindowType.UTILITY,
  Meta.WindowType.MODAL_DIALOG,
  Meta.WindowType.DIALOG,
  Meta.WindowType.NORMAL
];

function _filterWindow(x, index, array) {
  if (x && x.get_meta_window && x.get_meta_window().get_window_type) {
    return x.get_meta_window().get_window_type() !== -1;
  } else {
    return false;
  }
}

function _get_open_windows() {
  return global.get_window_actors()
    .filter(_filterWindow)
    .map(_strcut_data);
}

function get_open_windows() {
  return JSON.stringify(_get_open_windows());
}

function get_active_window() {
  const activeWindow = global.get_window_actors().find(x => x.get_meta_window().has_focus() && _filterWindow(x));
  return JSON.stringify(_strcut_data(activeWindow));
}

function get_icon(window_id) {
  const iconInfo = _get_icon(window_id);
  return JSON.stringify(iconInfo);
}

function _strcut_data(window_actor) {
  if (window_actor && window_actor.get_meta_window) {
    const _window = window_actor.get_meta_window();

    const process_id = _window.get_pid ? _window.get_pid() : 0;
    const info = _get_process_info(process_id);

    return {
      id: _window.get_id(),
      os: 'linux',
      info: {
        process_id,
        name: _window.get_wm_class ? _window.get_wm_class() : '',
        path: info.path,
        exec_name: info.exec_name,
      },
      title: _window.get_title ? _window.get_title() : '',
      position: {
        width: window_actor.get_width ? window_actor.get_width() : 0,
        height: window_actor.get_height ? window_actor.get_height() : 0,
        x: window_actor.get_x ? window_actor.get_x() : 0,
        y: window_actor.get_y ? window_actor.get_y() : 0,
        isFullScreen: _window.is_fullscreen(),
      },
      usage: { memory: _get_memory_usage(process_id) },
    };
  } else {
    return {
      id: 0,
      os: 'linux',
      title: '',
      info: {
        process_id: 0,
        path: '',
        exec_name: '',
      },
      position: {
        width: 0,
        height: 0,
        x: 0,
        y: 0,
        isFullScreen: false,
      },
      usage: { memory: 0 },
    };
  }
}

function _get_memory_usage(pid) {
  const [isOk, contents] = GLib.file_get_contents(`/proc/${pid}/statm`);
  if (isOk) {
    const value = String.fromCharCode.apply(null, contents);
    return parseInt(value.toString().split(' ')[0], 10);
  }
  return 0;
}

function _get_process_info(pid) {
  try {
    const path = GLib.file_read_link(`/proc/${pid}/exe`);
    if (path) {
      return {
        path,
        exec_name: path.split('/').pop(),
      };
    }
  } catch (e) { }
  return {
    path: '',
    exec_name: '',
  };
}"#;

// Javascript extension to get active and open window(s) informations
pub const GNOME_XWIN_EXTENSION_SCRIPT: &str = r#"const { Gio, GLib, Meta, Gtk: St, Shell } = imports.gi;

let _dbus = undefined;

function enable() {
  _dbus = Gio.DBusExportedObject.wrapJSObject(
    WaylandInterface,
    this,
  );
  _dbus.export(
    Gio.DBus.session,
    '/org/gnome/Shell/Extensions/XWinWaylandExtension',
  );
}

function disable() {
  _dbus.flush();
  _dbus.unexport();
  _dbus = undefined;
}

function init() {
  /** Do nothing */
}
"#;

pub const GNOME45_XWIN_EXTENSION_SCRIPT: &str = r#"import { Extension } from 'resource:///org/gnome/shell/extensions/extension.js';
import Gio from 'gi://Gio';
import GLib from 'gi://GLib';
import Meta from 'gi://Meta';
import St from "gi://St";
import Shell from "gi://Shell";

export default class XWinWaylandExtension extends Extension {

  _dbus = undefined;

  enable() {
    this._dbus = Gio.DBusExportedObject.wrapJSObject(
      WaylandInterface,
      this,
    );
    this._dbus.export(
      Gio.DBus.session,
      '/org/gnome/Shell/Extensions/XWinWaylandExtension',
    );
  }

  disable() {
    this._dbus.flush();
    this._dbus.unexport();
    this._dbus = undefined;
  }

  get_open_windows() {
    return get_open_windows();
  }

  get_active_window() {
    return get_active_window();
  }

  get_icon(window_id) {
    return get_icon(window_id);
  }
}
"#;

pub fn number_to_u32(value: &serde_json::Value) -> u32 {
  if value.is_number() {
    return value.as_u64().unwrap() as u32;
  }
  0
}

pub fn number_to_i32(value: &serde_json::Value) -> i32 {
  if value.is_number() {
    return value.as_i64().unwrap() as i32;
  }
  0
}

pub fn value_to_icon_info(response: &serde_json::Value) -> IconInfo {
  let response = response.as_object().unwrap();
  IconInfo {
    data: response["data"].as_str().unwrap().to_string(),
    height: number_to_u32(&response["height"]),
    width: number_to_u32(&response["width"]),
  }
}

pub fn value_to_window_info(response: &serde_json::Value) -> WindowInfo {
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
      is_full_screen: position["isFullScreen"].as_bool().unwrap(),
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
  }
}

pub struct GnomeVersion {
  pub version: u32,
  pub use_eval: bool,
}

impl GnomeVersion {
  fn new() -> Self {
    let version = get_gnome_version();
    let version: u32 = version.split(".").collect::<Vec<&str>>()[0]
      .parse()
      .unwrap_or(999);
    let use_eval = version < 41;
    Self { use_eval, version }
  }
}

pub static GNOME_SINGLETON: Lazy<Mutex<GnomeVersion>> =
  Lazy::new(|| Mutex::new(GnomeVersion::new()));
