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
  }
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
  }
}]
```

# Linux

> Dependencies are required to be installed for development purposes.

```sh
sudo apt install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-shm0-dev
```

# Darwin

## Screen recording permission introduced in macOS 10.15 Catalina

> macOS requires you to grant access for screen recording. If your project does not have it, the title will display `<unknown>` as its value.

<hr class="padding-top: 30px;padding-bottom:30px">

# Project References

**Project Inspirations:**

* [active-win](https://github.com/sindresorhus/active-win)
* [active-win-pos-rs](https://github.com/dimusic/active-win-pos-rs)
* This project was generated with [@napi-rs/cli](https://github.com/napi-rs)
