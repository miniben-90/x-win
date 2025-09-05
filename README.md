# @miniben90/x-win

[![ci-rs](https://github.com/miniben-90/x-win/actions/workflows/ci-rs.yml/badge.svg)](https://github.com/miniben-90/x-win/actions/workflows/ci-rs.yml)
[![ci-napi](https://github.com/miniben-90/x-win/actions/workflows/ci-napi.yml/badge.svg)](https://github.com/miniben-90/x-win/actions/workflows/ci-napi.yml)
[![Node version](https://img.shields.io/node/v/@miniben90/x-win.svg)](https://www.npmjs.com/package/@miniben90/x-win)
![npm type definitions](https://img.shields.io/npm/types/@miniben90/x-win)
![License](https://img.shields.io/npm/l/@miniben90/x-win)
![NPM Version](https://img.shields.io/npm/v/%40miniben90%2Fx-win)
[![Sponsor me!](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/miniben-90/button)

This project uses Rust and napi-rs to make it easy to obtain the active window or an array of open windows. It works on Microsoft Windows (10, 11), [Linux (with X server)](#linux), [Linux (with Gnome =< 45)](#wayland), and [macOS 10.6+](#darwin).

## How to install

```sh
# With npm
npm i @miniben90/x-win

# With yarn
yarn add @miniben90/x-win
```

## How to use x-win

### Get information about the currently active window

`exemple.ts`:

```typescript
import { activeWindow, type WindowInfo } from '@miniben90/x-win'

const currentWindow: WindowInfo = activeWindow()

console.log(currentWindow)
```

`response`:

```javascript
{
  id: 26148,
  info: {
    execName: "Code",
    name: "Visual Studio Code",
    path: "C:\\Users\\miniben\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe",
    processId: 26148
  },
  os: "win32",
  position: {
    height: 1048,
    width: 1936,
    x: -8,
    y: -8,
    isFullScreen: true
  },
  title: "● README.md - x-win - Visual Studio Code",
  usage: {
    memory: 113270784
  },
  url: ""
}
```

You can also use asynchronous methods instead. To do so, you just have to use `activeWindowAsync`, for example:

```typescript
import { activeWindowAsync, type WindowInfo } from '@miniben90/x-win'

activeWindow().then((currentWindow: WindowInfo) => {
  console.log(currentWindow)
})
```

### Get a list of open windows with information

`exemple.ts`:

```typescript
import { openWindows, type WindowInfo } from '@miniben90/x-win'

const windows: WindowInfo[] = openWindows()

console.log(windows)
```

`response`:

```javascript
;[
  {
    id: 26148,
    info: {
      execName: 'Code',
      name: 'Visual Studio Code',
      path: 'C:\\Users\\miniben\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe',
      processId: 26148,
    },
    os: 'win32',
    position: {
      height: 1048,
      width: 1936,
      x: -8,
      y: -8,
      isFullScreen: true,
    },
    title: '● README.md - x-win - Visual Studio Code',
    usage: {
      memory: 113270784,
    },
    url: '',
  },
]
```

You can also use asynchronous methods instead. To do so, you just have to use `openWindowsAsync`, for example:

```typescript
import { openWindowsAsync, type WindowInfo } from '@miniben90/x-win'

openWindowsAsync().then((windows: WindowInfo[]) => {
  console.log(windows)
})
```

### Subscribe to get the current active window

Thread will be start to check every `100ms` for a new active window (checking window title, window id and process id).

- `subscribeActiveWindow`: Create a subscription with a callback function
- `unsubscribeActiveWindow`: Remove a specific subscription
- `unsubscribeAllActiveWindow`: Remove all threads

```typescript
import { subscribeActiveWindow, unsubscribeActiveWindow, unsubscribeAllActiveWindow } from '@miniben90/x-win'

const a = subscribeActiveWindow((error, activeWindow) => {
  console.log('test a', activeWindow)
})

const b = subscribeActiveWindow((error, activeWindow) => {
  console.log('test b', activeWindow)
})

const c = subscribeActiveWindow((error, activeWindow) => {
  console.log('test c', activeWindow)
})

setTimeout(() => unsubscribeActiveWindow(c), 5000)

setTimeout(() => unsubscribeAllActiveWindow(), 10000)
```

### Get icon from `WindoInfo`

It is possible to get an icon from the `WindowInfo` class object using `getIcon` or `getIconAsync`, which will return an `IconInfo` struct.
The icon will be stored in `IconInfo.data` in base64 PNG format.

Example:

```typescript
import { activeWindow, type WindowInfo, type IconInfo } from '@miniben90/x-win'

const window: WindowInfo = activeWindow()
const iconInfo: IconInfo = window.getIcon()
console.log(console)
```

Response:

```javascript
{
  height: 32,
  width: 32,
  data: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAD/0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjvhfjvhfjvhfjvhXimW7+LvwJemhfNLvA6D34f/pp/H8Qz3fpdmH+dXeB1Hvw+/DX/dohneuq3Yf4Vth/0+px59JtC2+NfrTvz17rpw14GQDzT474W8yLoNq9l56aX55pHvQas7+DfJBboEV8mAPFMfupnmBdV2wOP/HvoUd8oAPFMfsIHm/9CevQ3C0A8kx//gea/kB7zrQIQz+THv7/5L6THfLsAxDP5ce9r/gvpsd8pAPFMftx7m/9Ceux3C0A8k//hPc1zsXkW869gkHgWieehF/teAYhn8t+/u3km29iQNjakDeZfRQJJSCCJEEjifnrx7xeAeCb//bsaINO0NK2ZcUpaMy2NDca8KISQoBRRQtQa1CJKiJBAoBf/QQGIZ/LfvbNtyEyGyQxj8nOrD+Kn73gU/x5vfdMTeYvZN9P3ha6KEkISeokfFoB4Jv/dO9pppjTrITlaTrz/rV/L+aPGv8epjcK3P+Qj2ZhXZl1QiggJveSPCkA8k//27W2bqZnlunFwNPEOT/xa/iP8xKM/io1FYd4XahUhoZf8cQGIZ/LfvK1tMzWzXDcOjibe4Ulfz3+En3jUR7AxL8xnhVpFSOilflIA4pn8N29t20zNLFeNg6OJd3jyN/If4Sce9WFszAvzWaEWESH0Uj8tAPFM/uu3tG2mZparxsGy8Q5P/kae2/c96IMoISJEiMvSkGkyzbs/41t4bj/xyA9lY16Yzwq1ipDQS/+sAMQz+a/ezDZMzSzXjYOjxts/8et5bj/2yA9jPiv0XVALl00NhjFZD8nbP/HreW4/+ZgPZ2NWmM+CWoMQ6GV+QQDimfyXb2AnTM0s142Do8bbPeEbeG4/8ZgPY2NWmM+CWgXANJn1kCzXyds87ut5bj/12A9nYx7M+6DWIAR6uV8TgHgm/+mr2IapJct1cnCUvM3jv53n9tOPfX825sG8D7oqAMbJrMfkaJW81T98O8/tZx77/mwsgnkf1CIihF7xjwQgnsl/9BJ/5eSlpzSrdXKwTN7qcd/Lc/u5F3tPFvNg3otaBMDUzHowy7V587//Hp7bz734e7IxC2a9qEWE9Nd61b97GQDxQrz8F/6BeS5//qmvJl6Il//CPzDP5c8/9dXE84f474X474X474X478U/AiZPaTD3cWUhAAAAAElFTkSuQmCC"
}
```

## Linux

Dependencies are required to be installed for development purposes.

```sh
sudo apt install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-shm0-dev pkg-config build-essential
```

> ⚠️**Warning**<br />
> libc.so.6 is needed<br />
> ⚠️**Warning**<br />
> Recovery url is not available on linux

### Wayland

In order to recover data, you'll need to install and activate an extension designed for systems running GNOME version 41 or newer (as evaluation is disabled from this version onward).

The extension can be installed using the `installExtension()` function, which will deposit it in `~/.local/share/gnome-shell/extensions/x-win@miniben90.org`.

After executing this function, it's vital to **restart the user session** to load the new extension and then proceed to enable it using `enableExtension()` to be able to use x-win.

It is possible to know if the extension is installed or / and enabled using `isInstalledExtension()` and `isEnabledExtension()`.

| Gnome Distrib.         | Tested |
| ---------------------- | ------ |
| Fedora Workstation 39  | ✅     |
| Ubuntu 22.04.4 Desktop | ✅     |
| Debian 12 Desktop      | ✅     |

## Darwin

> ⚠️**Warning**<br /> This project work only for macos version 10.6+

## Screen recording permission introduced in macOS 10.15 Catalina

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

## Electron

- To prevent potential crashes and issues, it's recommended to execute recovery operations within a worker thread ([https://nodejs.org/api/worker_threads.html](https://nodejs.org/api/worker_threads.html))
- For macOS, you can utilize functions to check and request screen permissions ([https://www.electronjs.org/fr/docs/latest/api/system-preferences](https://www.electronjs.org/fr/docs/latest/api/system-preferences))
- When building your application, it’s important to specify all optional packages to be moved to `asar.unpack` to prevent issues during the signing process. [https://www.electronforge.io/config/plugins/auto-unpack-natives](https://www.electronforge.io/config/plugins/auto-unpack-natives) [https://www.electron.build/app-builder-lib.interface.configuration#asarunpack](https://www.electron.build/app-builder-lib.interface.configuration#asarunpack)

### Questions

- Why can't I find optional dependencies when I build my application?

When setting up CI/CD to build your application, it's important to enforce the installation of sub-packages for each system to prevent any missing components.

examples:

- `npm install --save-dev @miniben90/x-win-win32-x64-msvc @miniben90/x-win-win32-ia32-msvc @miniben90/x-win-win32-arm64-msvc --ignore-platform --ignore-engines` for win32 system
- `npm install --save-dev @miniben90/x-win-darwin-arm64 @miniben90/x-win-darwin-universal @miniben90/x-win-darwin-x64 --ignore-platform --ignore-engines` for Darwin system
- `npm install --save-dev @miniben90/x-win-linux-x64-gnu @miniben90/x-win-linux-x64-musl4 --ignore-platform --ignore-engines` for Linux system

<hr class="padding-top: 30px;padding-bottom:30px">

## Project References

**Project Inspirations:**

- [active-win](https://github.com/sindresorhus/active-win)
- [active-win-pos-rs](https://github.com/dimusic/active-win-pos-rs)
- This project was generated with [@napi-rs/cli](https://github.com/napi-rs)
