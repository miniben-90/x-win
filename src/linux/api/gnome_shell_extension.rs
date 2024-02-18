#![allow(unused_imports)]

// Metadata for gnome version > then 41
const GNOME_XWIN_EXTENSION_META: &str = r#"
{
  "name": "@mininben90/x-win extended",
  "description": "Get active and open window(s) informations for @miniben90/x-win node package.",
  "uuid": "x-win@miniben90.org",
  "shell-version": [ "41", "42", "43", "44" ],
  "url": "https://github.com/miniben-90/x-win.git",
  "version-name": "1.8.0"
}
"#;

// Javascript extension to get active and open window(s) informations
const GNOME_XWIN_EXTENSION_SCRIPT: &str = r#"
const { Gio, GLib, Meta } = imports.gi;

const WaylandInterface = `
<node>
  <interface name="org.gnome.Shell.Extensions.XWinWaylandExtension">
    <method name="get_active_window">
      <arg name="get_active_window" type="object" direction="out" />
    </method>
    <method name="get_open_windows">
      <arg name="get_open_windows" type="object" direction="out" />
    </method>
  </interface>
</node>
`;

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

const AllowedWindow = [
  Meta.WindowType.DESKTOP,
  Meta.WindowType.OVERRIDE_OTHER,
  Meta.WindowType.UTILITY,
  Meta.WindowType.MODAL_DIALOG,
  Meta.WindowType.DIALOG,
  Meta.WindowType.NORMA
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

function _strcut_data(window_actor) {
  if (window_actor && window_actor.get_meta_window) {
    const _window = window_actor.get_meta_window();

    const process_id = _window.get_pid ? _window.get_pid() : 0;
    const info = _get_process_info(process_id);

    return Object{
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
      },
      url: '',
      usage: { memory: _get_memory_usage(process_id) },
    };
  } else {
    return {
      id: 0,
      os: 'linux',
      title: '',
      url: '',
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
      },
      usage: { memory: 0 },
    };
  }
}

function _get_memory_usage(pid) {
  const [isOk, contents] = GLib.file_get_contents(`/proc/${pid}/statm`);
  if (isOk) {
    return contents.toString().split(' ')[0];
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
}

function init() {
  /** Do nothing */
}
"#;
