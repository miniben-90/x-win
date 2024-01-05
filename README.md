[![CI](https://github.com/miniben-90/x-win/actions/workflows/CI.yml/badge.svg)](https://github.com/miniben-90/x-win/actions/workflows/CI.yml)
[![Node version](https://img.shields.io/node/v/@miniben90/x-win.svg)](https://www.npmjs.com/package/@miniben90/x-win)
![npm type definitions](https://img.shields.io/npm/types/@miniben90/x-win)

# @miniben90/x-win

This project uses Rust and napi-rs to make it easy to obtain the active window or an array of open windows. It works on Microsoft Windows (10, 11), [Linux (with X server)](#linux), and [macOS](#darwin).

## How to install

```sh
# With npm
npm i @miniben90/x-win

# With yarn
yarn add @miniben90/x-win
```

## How to use

### Get information about the currently active window

`exemple.ts`:

```typescript
import { activeWindow, type WindowInfo } from '@miniben90/x-win';

const currentWindow: WindowInfo = activeWindow();

console.log(currentWindow);
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
    y: -8
  },
  title: "● README.md - x-win - Visual Studio Code",
  usage: {
    memory: 113270784
  },
  url: ""
}
```

### Get a list of open windows with information

`exemple.ts`:

```typescript
import { openWindows, type WindowInfo } from '@miniben90/x-win';

const windows: WindowInfo[] = openWindows();

console.log(windows);
```

`response`:

```javascript
[{
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
    y: -8
  },
  title: "● README.md - x-win - Visual Studio Code",
  usage: {
    memory: 113270784
  },
  url: ""
}]
```

### Subscribe to get the current active window

Thread will be start to check every `100ms` for a new active window (checking window title, window id and process id).

* `subscribeActiveWindow`: Create a subscription with a callback function
* `unsubscribeActiveWindow`: Remove a specific subscription
* `unsubscribeAllActiveWindow`: Remove all threads


```typescript
import { subscribeActiveWindow, unsubscribeActiveWindow, unsubscribeAllActiveWindow } from '@miniben90/x-win';

const a = subscribeActiveWindow(activeWindow => {
  console.log('test a', activeWindow);
});

const b = subscribeActiveWindow(activeWindow => {
  console.log('test b', activeWindow);
});


const c = subscribeActiveWindow(activeWindow => {
  console.log('test c', activeWindow);
});

setTimeout(() => unsubscribeActiveWindow(c), 5000);

setTimeout(() => unsubscribeAllActiveWindow(), 10000);
```

# Linux

Dependencies are required to be installed for development purposes.

```sh
sudo apt install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-shm0-dev
```

> ⚠️**Warning**
> Recovery url is not available on linux

# Darwin

## Screen recording permission introduced in macOS 10.15 Catalina

> macOS requires you to grant access for screen recording. If your project does not have it, the title will be an empty value.

# URLs (Only available for Darwin and Windows Systems)

It is possible to get URL of browsers window for macOS and Windows.

## Windows

| Browser name | Tested |
|---|---|
| firefox | ✅ |
| firefox developer edition | ✅ |
| google chrome | ✅ |
| microsoft edge | ✅ |
| opera software - opera | ✅ |
| opera software - opera GX | ✅ |
| brave |  |
| vivaldi |  |
| iron |  |
| epic |  |
| chromium | ✅ |
| ucozmedia |  |
| blisk |  |
| maxthon |  |
| beaker |  |
| beaker browser |  |

## macOS

It will use AppleScript to get informations for chromium browsers and safari

***For the moment Firefox and firefox developer edition are not supported***

| Browser name | Tested |
|---|---|
| Safari | ✅ |
| Safari Technology Preview |  |
| google Chrome | ✅ |
| google Chrome beta |  |
| google Chrome dev |  |
| google Chrome canary |  |
| brave Browser |  |
| brave Browser beta |  |
| brave Browser nightly |  |
| microsoft edge | ✅ |
| microsoft edge Beta |  |
| microsoft edge Dev |  |
| microsoft edge Canary |  |
| mighty |  |
| ghost browser |  |
| bookry wavebox |  |
| pushplaylabs sidekick |  |
| opera software - Opera | ✅ |
| opera software - OperaNext |  |
| opera software - OperaDeveloper |  |
| opera software - OperaGX | ✅ |
| Vivaldi |  |

## Electron

* To prevent potential crashes and issues, it's recommended to execute recovery operations within a worker thread ([https://nodejs.org/api/worker_threads.html](https://nodejs.org/api/worker_threads.html))
* For macOS, you can utilize functions to check and request screen permissions ([https://www.electronjs.org/fr/docs/latest/api/system-preferences](https://www.electronjs.org/fr/docs/latest/api/system-preferences))

<hr class="padding-top: 30px;padding-bottom:30px">

# Project References

**Project Inspirations:**

* [active-win](https://github.com/sindresorhus/active-win)
* [active-win-pos-rs](https://github.com/dimusic/active-win-pos-rs)
* This project was generated with [@napi-rs/cli](https://github.com/napi-rs)
