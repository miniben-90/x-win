# x-win

[![CI](https://github.com/miniben-90/x-win/actions/workflows/ci-rs.yml/badge.svg)](https://github.com/miniben-90/x-win/actions/workflows/ci-rs.yml)
[![crates.io version](https://img.shields.io/crates/v/x-win.svg)](https://crates.io/crates/x-win)
[![docs](https://docs.rs/x-win/badge.svg)](https://docs.rs/x-win/)
![License](https://img.shields.io/crates/l/x-win)
[![Sponsor me!](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/miniben-90/button)

This package make it easy to obtain the active window or an array of open windows. It works on Microsoft Windows (10, 11), [Linux (with X server)](#linux), [Linux (with Gnome =< 45)](#gnome), and [macOS 10.6+](#darwin).

## Get information about the currently active window

`examples/get_active_window.rs`:

```rust
use x_win::get_active_window;

fn main() {
  match get_active_window() {
    Ok(active_window) => {
      println!("active window: {:#?}", active_window);
    }
    Err(_) => {
      println!("error occurred while getting the active window");
    }
  }
}
```

`response`:

```log
active window: WindowInfo {
    id: 23624,
    os: "win32",
    title: "● README.md - x-win - Visual Studio Code",
    position: WindowPosition {
        x: -8,
        y: -8,
        width: 1936,
        height: 1048,
        is_full_screen: true,
    },
    info: ProcessInfo {
        process_id: 23624,
        path: "C:\\Users\\miniben\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe",
        name: "Code",
        exec_name: "Code",
    },
    usage: UsageInfo {
        memory: 129138688,
    },
    url: "",
}
```

## Get a list of open windows with information

`examples/get_active_window.rs`:

```rust
use x_win::get_open_windows;

fn main() {
  match get_open_windows() {
    Ok(open_windows) => {
      println!("open windows: {:#?}", open_windows);
    }
    Err(_) => {
      println!("error occurred while getting open windows");
    }
  }
}
```

`response`:

```log
open windows: [
    WindowInfo {
        id: 23624,
        os: "win32",
        title: "● README.md - x-win - Visual Studio Code",
        position: WindowPosition {
            x: -8,
            y: -8,
            width: 1936,
            height: 1048,
            is_full_screen: true,
        },
        info: ProcessInfo {
            process_id: 23624,
            path: "C:\\Users\\miniben\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe",
            name: "Code",
            exec_name: "Code",
        },
        usage: UsageInfo {
            memory: 128770048,
        },
        url: "",
    },
]
```

## Get icon from `WindoInfo`

`examples/get_window_icon.rs`:

```rust
use x_win::get_active_window, get_window_icon;

fn main() {
  match get_active_window() {
    Ok(active_window) => match get_window_icon(&active_window) {
      Ok(icon_info) => {
        println!("icon info: {:#?}", icon_info);
      }
      Err(_) => {
        println!("error occurred while getting the icon info of active window");
      }
    },
    Err(_) => {
      println!("error occurred while getting the active window");
    }
  }
}
```

`response`:

```log
icon info: IconInfo {
    data: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAFLElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovhP8GKf92uvOj7skV+7bHpwUTG7+99/64c/+mN4Xoj/YC/2eb/2qk8cy69MsNXNZzz0JV8KgGm9bj574Uee9sGPeDeeDfEf6LFf+Jvv/aSlv26CLZ7pka/4SjxQW69bnrvwI0//4Ee8G4D4D/Lgz/7Vj7x1Kl/Dc3nkK70iz8+0Glq54+43EP8BHvRZv/oRz5jia3k+HvlKr8QLkkeru8S/06nP+NWfON/0trwAj3ylV+aFQPw7nPz0X/3xC6m344V4xCu+Mi8E4t/gEZ/xi6fuy/LDl1Kvz7/gEa/4KrwQCODUV//Dt5+X3g9g0/7TG+w3ffLHvPh5no9HfMYvnrp9Kr+/Mo/mRfCIV3pVXgh06qv+4dvOi/fnAQoc3GS/4zM+5sV/iQd4xGf84qnbx/j9lfVo/gUFHzx0u/sMHvuKX8ULhja+6u//+Ai9Es/HafvLz33si38CwIM+9Rdf8e4sPzWkb+BfMA+ecG3ke936BW/yD4/4yYMDXjC09ZV//ysH8Ia8AIv0H19zePYzbz9c/0Sabf4Fx4p/7JqOD3ny573pedubj/iJ/QNeMPSIr/r7U7clv7eGx/CCZEsunA2mgRckYP908Sfd98Vv9k08k+2NR/zE/iEvGAJ4xFf9/anbmn9vjR7DC7N3EY72eW4z+fGnw+9955e8+Z/yALY3HvHj+4e8YIgH2P6yv/uRfXhHXpjVEi6dBycA28GPbqt9yF1f+pYXeC62Fw//sf0jXjDEc9n60r/94QP0TrwwbYLdcxyflp+8+xVv+SW8ALYXD//R/SNeMMRz2fz4n/uhw82T78zGNv+SnfQn733KS30JL4Dt+cN/ZH/JC4Z4gM2P+7kfOkzeGYDFFuyc4l+yCT98DD7srk9+yQs8F9vzh//I/pIXDAHc8Ak/e/LsxO+O5sV4oMUW7JzmX9KJfzhmv/e5T3mpP+cBbM8e9sP7K14wdMMn/OzJs6N/d7RejOenn8PxM4kieCEEe8fEJ+x+ykt9K89ke/awH9pf8YKh2Uf/zC+skzflBejkP9g6dcNn7pbZTxl2+Bcs4IdOFD78rk9+qQu2+4f94N6aFwx1H/XTvz+aV+P52BBfePQ1b/1pAMc//69fbl/66WbfxL+gir/fMu998dNe6u8e9oN7a14wNP/In/qGlfWhPIBgb7v47fa++m1+nQfY+dy/OnEk/e4EL86/QGLveNUnHr/pwd/MC4YAZh/xU1+/Nh8GUMXvb8hvufe1b3uR52Pnc//qxKH0uw1enBfBQx78EF4IxL/Bzuf+1YklfO8Ib86/4CEPfigvBOLfofucv/q+Ed6dF+IhD34oLwTi36l+1l9+3yTenRfgwQ95GC8E4j9A/1l/+b4DfAfPx4Mf8jBekDaMt4n/IN1n/uX7jvAdPJcHP+ShPD9tatNyf+91xX+g+hl/8TYTfBdwjGd68EMexgO1aZqWR0ffe+6jb3k/APEfrH7GX7zMZH4LOFZrx4233AJAG6dptTz63nMffcv78WyI/wyf+ucvfc1113210aNLhMdx+O7zH/OgT+F58Y/bubsQq+kGhwAAAABJRU5ErkJggg==",
    height: 32,
    width: 32,
}
```

## Linux

Dependencies are required to be installed for development purposes.

```sh
sudo apt install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-shm0-dev pkg-config build-essential
```

> ⚠️**Warning**
> libc.so.6 is needed

> ⚠️**Warning**
> Recovery url is not available on linux

## GNOME

> Gnome using wayland.

In order to recover data, you'll need to install and activate an extension designed for systems running GNOME version 41 or newer (as evaluation is disabled from this version onward).

The extension can be installed using the `x_win::install_extension()` function, which will deposit it in `~/.local/share/gnome-shell/extensions/x-win@miniben90.org`.

After executing this function, it's vital to **restart the user session** to load the new extension and then proceed to enable it using `x_win::enable_extension()` to be able to use x-win.

It is possible to know if the extension is installed or / and enabled using `x_win::is_installed_extension()` and `x_win::is_enabled_extension()`.

| Gnome Distrib.         | Tested |
| ---------------------- | ------ |
| Fedora Workstation 39  | ✅     |
| Ubuntu 22.04.4 Desktop | ✅     |
| Debian 12 Desktop      | ✅     |

## Darwin

> This package can be use only with darwin version 10.6+.

### Screen recording permission introduced in macOS 10.15 Catalina

> macOS requires you to grant access for screen recording. If your project does not have it, the title will be an empty value.

## URLs (Only available for Darwin and Windows Systems)

It is possible to get URL of browsers window for macOS and Windows.

### Windows

| Browser name              | Tested |
| ------------------------- | ------ |
| firefox                   | ✅     |
| firefox developer edition | ✅     |
| google chrome             | ✅     |
| microsoft edge            | ✅     |
| opera software - opera    | ✅     |
| opera software - opera GX | ✅     |
| brave                     |        |
| vivaldi                   |        |
| iron                      |        |
| epic                      |        |
| chromium                  | ✅     |
| ucozmedia                 |        |
| blisk                     |        |
| maxthon                   |        |
| beaker                    |        |
| beaker browser            |        |
| LibreWolf                 | ✅     |

### macOS

It will use AppleScript to get informations for chromium browsers and safari

**_For the moment Firefox and firefox developer edition are not supported_**

| Browser name                    | Tested |
| ------------------------------- | ------ |
| Safari                          | ✅     |
| Safari Technology Preview       |        |
| google Chrome                   | ✅     |
| google Chrome beta              |        |
| google Chrome dev               |        |
| google Chrome canary            |        |
| brave Browser                   |        |
| brave Browser beta              |        |
| brave Browser nightly           |        |
| microsoft edge                  | ✅     |
| microsoft edge Beta             |        |
| microsoft edge Dev              |        |
| microsoft edge Canary           |        |
| mighty                          |        |
| ghost browser                   |        |
| bookry wavebox                  |        |
| pushplaylabs sidekick           |        |
| opera software - Opera          | ✅     |
| opera software - OperaNext      |        |
| opera software - OperaDeveloper |        |
| opera software - OperaGX        | ✅     |
| Vivaldi                         |        |

<hr class="padding-top: 30px;padding-bottom:30px">

## Project References

**Project Inspirations:**

- [active-win](https://github.com/sindresorhus/active-win)
- [active-win-pos-rs](https://github.com/dimusic/active-win-pos-rs)
- This project was generated with [@napi-rs/cli](https://github.com/napi-rs)
