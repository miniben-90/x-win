/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null
let localFilePath = {}
let filePath = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

function setLocalFilePath(platform, arch, filePath) {
  localFilePath[platform] = localFilePath[platform] || {};
  localFilePath[platform][arch] = filePath
}

function getLocalFilePath() {
  localFilePath[platform] = localFilePath[arch] || {};
  return localFilePath[platform][arch]
}

function init() {
  switch (platform) {
    case 'android':
      switch (arch) {
        case 'arm64':
          localFileExisted = existsSync(join(__dirname, 'x-win.android-arm64.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./x-win.android-arm64.node')
            } else {
              nativeBinding = require('@miniben90/x-win-android-arm64')
            }
          } catch (e) {
            loadError = e
          }
          break
        case 'arm':
          localFileExisted = existsSync(join(__dirname, 'x-win.android-arm-eabi.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./x-win.android-arm-eabi.node')
            } else {
              nativeBinding = require('@miniben90/x-win-android-arm-eabi')
            }
          } catch (e) {
            loadError = e
          }
          break
        default:
          throw new Error(`Unsupported architecture on Android ${arch}`)
      }
      break
    case 'win32':
      switch (arch) {
        case 'x64':
          filePath = getLocalFilePath() || join(__dirname, 'x-win.win32-x64-msvc.node')
          localFileExisted = existsSync(filePath)
          try {
            if (localFileExisted) {
              nativeBinding = require(filePath)
            } else {
              nativeBinding = require('@miniben90/x-win-win32-x64-msvc')
            }
          } catch (e) {
            loadError = e
          }
          break
        case 'ia32':
          localFileExisted = existsSync(
            join(__dirname, 'x-win.win32-ia32-msvc.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./x-win.win32-ia32-msvc.node')
            } else {
              nativeBinding = require('@miniben90/x-win-win32-ia32-msvc')
            }
          } catch (e) {
            loadError = e
          }
          break
        case 'arm64':
          filePath = getLocalFilePath() || join(__dirname, 'x-win.win32-arm64-msvc.node')
          localFileExisted = existsSync(filePath)
          try {
            if (localFileExisted) {
              nativeBinding = require(filePath)
            } else {
              nativeBinding = require('@miniben90/x-win-win32-arm64-msvc')
            }
          } catch (e) {
            loadError = e
          }
          break
        default:
          throw new Error(`Unsupported architecture on Windows: ${arch}`)
      }
      break
    case 'darwin':
      localFileExisted = existsSync(join(__dirname, 'x-win.darwin-universal.node'))
      try {
        if (localFileExisted) {
          nativeBinding = require('./x-win.darwin-universal.node')
        } else {
          nativeBinding = require('@miniben90/x-win-darwin-universal')
        }
        break
      } catch {}
      switch (arch) {
        case 'x64':
          localFileExisted = existsSync(join(__dirname, 'x-win.darwin-x64.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./x-win.darwin-x64.node')
            } else {
              nativeBinding = require('@miniben90/x-win-darwin-x64')
            }
          } catch (e) {
            loadError = e
          }
          break
        case 'arm64':
          localFileExisted = existsSync(
            join(__dirname, 'x-win.darwin-arm64.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./x-win.darwin-arm64.node')
            } else {
              nativeBinding = require('@miniben90/x-win-darwin-arm64')
            }
          } catch (e) {
            loadError = e
          }
          break
        default:
          throw new Error(`Unsupported architecture on macOS: ${arch}`)
      }
      break
    case 'freebsd':
      if (arch !== 'x64') {
        throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
      }
      localFileExisted = existsSync(join(__dirname, 'x-win.freebsd-x64.node'))
      try {
        if (localFileExisted) {
          nativeBinding = require('./x-win.freebsd-x64.node')
        } else {
          nativeBinding = require('@miniben90/x-win-freebsd-x64')
        }
      } catch (e) {
        loadError = e
      }
      break
    case 'linux':
      switch (arch) {
        case 'x64':
          if (isMusl()) {
            localFileExisted = existsSync(
              join(__dirname, 'x-win.linux-x64-musl.node')
            )
            try {
              if (localFileExisted) {
                nativeBinding = require('./x-win.linux-x64-musl.node')
              } else {
                nativeBinding = require('@miniben90/x-win-linux-x64-musl')
              }
            } catch (e) {
              loadError = e
            }
          } else {
            localFileExisted = existsSync(
              join(__dirname, 'x-win.linux-x64-gnu.node')
            )
            try {
              if (localFileExisted) {
                nativeBinding = require('./x-win.linux-x64-gnu.node')
              } else {
                nativeBinding = require('@miniben90/x-win-linux-x64-gnu')
              }
            } catch (e) {
              loadError = e
            }
          }
          break
        case 'arm64':
          if (isMusl()) {
            localFileExisted = existsSync(
              join(__dirname, 'x-win.linux-arm64-musl.node')
            )
            try {
              if (localFileExisted) {
                nativeBinding = require('./x-win.linux-arm64-musl.node')
              } else {
                nativeBinding = require('@miniben90/x-win-linux-arm64-musl')
              }
            } catch (e) {
              loadError = e
            }
          } else {
            localFileExisted = existsSync(
              join(__dirname, 'x-win.linux-arm64-gnu.node')
            )
            try {
              if (localFileExisted) {
                nativeBinding = require('./x-win.linux-arm64-gnu.node')
              } else {
                nativeBinding = require('@miniben90/x-win-linux-arm64-gnu')
              }
            } catch (e) {
              loadError = e
            }
          }
          break
        case 'arm':
          if (isMusl()) {
            localFileExisted = existsSync(
              join(__dirname, 'x-win.linux-arm-musleabihf.node')
            )
            try {
              if (localFileExisted) {
                nativeBinding = require('./x-win.linux-arm-musleabihf.node')
              } else {
                nativeBinding = require('@miniben90/x-win-linux-arm-musleabihf')
              }
            } catch (e) {
              loadError = e
            }
          } else {
            localFileExisted = existsSync(
              join(__dirname, 'x-win.linux-arm-gnueabihf.node')
            )
            try {
              if (localFileExisted) {
                nativeBinding = require('./x-win.linux-arm-gnueabihf.node')
              } else {
                nativeBinding = require('@miniben90/x-win-linux-arm-gnueabihf')
              }
            } catch (e) {
              loadError = e
            }
          }
          break
        case 'riscv64':
          if (isMusl()) {
            localFileExisted = existsSync(
              join(__dirname, 'x-win.linux-riscv64-musl.node')
            )
            try {
              if (localFileExisted) {
                nativeBinding = require('./x-win.linux-riscv64-musl.node')
              } else {
                nativeBinding = require('@miniben90/x-win-linux-riscv64-musl')
              }
            } catch (e) {
              loadError = e
            }
          } else {
            localFileExisted = existsSync(
              join(__dirname, 'x-win.linux-riscv64-gnu.node')
            )
            try {
              if (localFileExisted) {
                nativeBinding = require('./x-win.linux-riscv64-gnu.node')
              } else {
                nativeBinding = require('@miniben90/x-win-linux-riscv64-gnu')
              }
            } catch (e) {
              loadError = e
            }
          }
          break
        case 's390x':
          localFileExisted = existsSync(
            join(__dirname, 'x-win.linux-s390x-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./x-win.linux-s390x-gnu.node')
            } else {
              nativeBinding = require('@miniben90/x-win-linux-s390x-gnu')
            }
          } catch (e) {
            loadError = e
          }
          break
        default:
          throw new Error(`Unsupported architecture on Linux: ${arch}`)
      }
      break
    default:
      throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
  }
  
  if (!nativeBinding) {
    if (loadError) {
      throw loadError
    }
    throw new Error(`Failed to load native binding`)
  }
}

const { WindowInfo, activeWindow, activeWindowAsync, openWindows, openWindowsAsync, subscribeActiveWindow, unsubscribeActiveWindow, unsubscribeAllActiveWindow, installExtension, uninstallExtension, enableExtension, disableExtension } = nativeBinding

module.exports.WindowInfo = WindowInfo
module.exports.activeWindow = activeWindow
module.exports.activeWindowAsync = activeWindowAsync
module.exports.openWindows = openWindows
module.exports.openWindowsAsync = openWindowsAsync
module.exports.subscribeActiveWindow = subscribeActiveWindow
module.exports.unsubscribeActiveWindow = unsubscribeActiveWindow
module.exports.unsubscribeAllActiveWindow = unsubscribeAllActiveWindow
module.exports.installExtension = installExtension
module.exports.uninstallExtension = uninstallExtension
module.exports.enableExtension = enableExtension
module.exports.disableExtension = disableExtension
module.exports.setLocalFilePath = setLocalFilePath
module.exports.init = init
