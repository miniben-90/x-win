[![CI](https://github.com/miniben-90/x-win/actions/workflows/CI.yml/badge.svg)](https://github.com/miniben-90/x-win/actions/workflows/CI.yml)
[![Node version](https://img.shields.io/node/v/@miniben90/x-win.svg)](https://www.npmjs.com/package/@miniben90/x-win)
![npm type definitions](https://img.shields.io/npm/types/@miniben90/x-win)

# @miniben90/x-win

> This project work with Rust and napi-rs to make easy to have active window or an array of open windows.
> It work with Microsoft Windows (10, 11), Linux (With Xserver) and Macos(*).

## How to install

Easy to install:

```sh
# With npm
npm i @miniben90/x-win

# With yarn
yarn add @miniben90/x-win
```

## How to use

### Get information of current active window

`exemple.ts`:
```typescript
import { activeWindow, type WindowInfo } from '@miniben90/x-win';

const currentWindow: WindowInfo = activeWindow();

console.log(currentWindow);
```

`response`:
```json
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

### Get List of open windows with informations

`exemple.ts`:
```typescript
import { openWindows, type WindowInfo } from '@miniben90/x-win';

const windows: WindowInfo[] = openWindows();

console.log(windows);
```

`response`:
```json
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


## For Darwin

> ### Record screen required for macos
> Macos require to grant access for recording screen. If your project don't have it the title will have `<unknown>` as value.

<hr class="padding-top: 30px;padding-bottom:30px">

**This project was inspired by:**
* [active-win](https://github.com/sindresorhus/active-win)
* [active-win-pos-rs](https://github.com/dimusic/active-win-pos-rs)
* This project was generated with [@napi-rs/cli](https://github.com/napi-rs)
