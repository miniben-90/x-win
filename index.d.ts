/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/**
 * Struct to store process information of the window
*/
export interface IconInfo {
  data: string
  height: number
  width: number
}
/**
 * Struct to store process information of the window
*/
export interface ProcessInfo {
  processId: number
  path: string
  name: string
  execName: string
}
/**
 * Struct to store usage data of the window
*/
export interface UsageInfo {
  memory: number
}
/**
 * Struct to store position and size of the window
*/
export interface WindowPosition {
  x: number
  y: number
  width: number
  height: number
  isFullScreen: boolean
}
/**
 * Retrieve information the about currently active window.
 * Returns an object of `WindowInfo`.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { activeWindow } = require('@miniben90/x-win');
 *
 * const currentWindow = activeWindow();
 * console.log(currentWindow);
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { activeWindow } from '@miniben90/x-win';
 *
 * const currentWindow = activeWindow();
 * console.log(currentWindow);
 * ```
 *
 * # Information about Electron
 *
 * It is recommended to use this function within a worker to mitigate potential recovery issues on MacOS.
*/
export declare function activeWindow(): WindowInfo
/**
 * Retrieve information about the currently active window as a promise.
 * Returns an object of `WindowInfo`.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * activeWindowAsync()
 * .then(currentWindow => {
 *   console.log(currentWindow);
 * });
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { activeWindowAsync } from '@miniben90/x-win';
 *
 * activeWindowAsync()
 * .then(currentWindow => {
 *   console.log(currentWindow);
 * });
 * ```
 *
 * # Information about Electron
 *
 * It is recommended to use this function within a worker to mitigate potential recovery issues on MacOS.
*/
export declare function activeWindowAsync(): Promise<WindowInfo>
/**
 * Retrieve information about the currently open windows.
 * Returns an array of `WindowInfo`, each containing details about a specific open window.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { openWindows } = require('@miniben90/x-win');
 *
 * const windows = openWindows();
 * for (let i = 0; i < windows.length; i++) {
 *   console.log(i, windows[i]);
 * }
 * ```
 *
 * ## Typescript Example
 *
 * ```typescript
 * import { openWindows } from '@miniben90/x-win';
 *
 * const windows = openWindows();
 * for (let i = 0; i < windows.length; i++) {
 *   console.log(i, windows[i]);
 * }
 * ```
 *
 * # Information about Electron
 *
 * It is recommended to use this function within a worker to mitigate potential recovery issues on MacOS.
*/
export declare function openWindows(): Array<WindowInfo>
/**
 * Retrieve information about the currently open windows as a promise.
 * Returns an array of `WindowInfo`, each containing details about a specific open window.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { openWindowsAsync } = resuire('@miniben90/x-win');
 *
 * openWindowsAsync()
 * .then(windows => {
 *   for (let i = 0; i < windows.length; i++) {
 *     console.log(i, windows[i]);
 *   }
 * });
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { openWindowsAsync } from '@miniben90/x-win';
 *
 * openWindowsAsync()
 * .then(windows => {
 *   for (let i = 0; i < windows.length; i++) {
 *     console.log(i, windows[i]);
 *   }
 * });
 * ```
 *
 * # Information about Electron
 *
 * It is recommended to use this function within a worker to mitigate potential recovery issues on MacOS.
*/
export declare function openWindowsAsync(): Promise<Array<WindowInfo>>
/**
 * Subscribe an observer thread to monitor changes in the active window.
 * @param {function} callback - Callback function that returns the active window when it changes
 * @param {number} [interval=100] - Interval between checks for changes in the active window (default: 100ms)
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { subscribeActiveWindow, unsubscribeAllActiveWindow } = require('@miniben90/x-win');
 *
 * const a = subscribeActiveWindow((err, info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((err, info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((err, info) => {
 *   t.log(c, info);
 * });
 * const d = subscribeActiveWindow((err, info) => {
 *   t.log(c, info);
 * },500);// sleep interval: 500ms
 *
 * unsubscribeAllActiveWindow();
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { subscribeActiveWindow, unsubscribeAllActiveWindow } from '@miniben90/x-win';
 *
 * const a = subscribeActiveWindow((err, info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((err, info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((err, info) => {
 *   t.log(c, info);
 * });
 * const d = subscribeActiveWindow((err, info) => {
 *   t.log(c, info);
 * },500);// sleep interval: 500ms
 *
 * unsubscribeAllActiveWindow();
 * ```
 *
*/
export declare function subscribeActiveWindow(callback: (error: Error | null, info: WindowInfo | undefined) => void, interval?: number): number
/**
 * Terminate and unsubscribe a specific observer using their ID.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { subscribeActiveWindow, unsubscribeActiveWindow } = require('@miniben90/x-win');
 *
 * const a = subscribeActiveWindow((info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((info) => {
 *   t.log(c, info);
 * });
 *
 * unsubscribeActiveWindow(a);
 * unsubscribeActiveWindow(b);
 * unsubscribeActiveWindow(c);
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { subscribeActiveWindow, unsubscribeActiveWindow } from '@miniben90/x-win';
 *
 * const a = subscribeActiveWindow((info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((info) => {
 *   t.log(c, info);
 * });
 *
 * unsubscribeActiveWindow(a);
 * unsubscribeActiveWindow(b);
 * unsubscribeActiveWindow(c);
 * ```
*/
export declare function unsubscribeActiveWindow(threadId: number): void
/**
 * Terminate and unsubscribe all observer threads monitoring changes in the active window.
 *
 * # Example
 *
 * ## Javascript example
 *
 * ```javascript
 * const { subscribeActiveWindow, unsubscribeAllActiveWindow } = require('@miniben90/x-win');
 *
 * const a = subscribeActiveWindow((info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((info) => {
 *   t.log(c, info);
 * });
 *
 * unsubscribeAllActiveWindow();
 * ```
 *
 * ## Typescript example
 *
 * ```typescript
 * import { subscribeActiveWindow, unsubscribeAllActiveWindow } from '@miniben90/x-win';
 *
 * const a = subscribeActiveWindow((info) => {
 *   t.log(a, info);
 * });
 * const b = subscribeActiveWindow((info) => {
 *   t.log(b, info);
 * });
 * const c = subscribeActiveWindow((info) => {
 *   t.log(c, info);
 * });
 *
 * unsubscribeAllActiveWindow();
 * ```
*/
export declare function unsubscribeAllActiveWindow(): void
/**
 * Install "@mininben90/x-win" Gnome extension required for Linux using Gnome > 41.
 * This function will write extension files needed to correctly detect working windows with Wayland desktop environment.
 * **Restart session will be require to install the gnome extension.**
*/
export declare function installExtension(): boolean
/**
 * Uninstall "@mininben90/x-win" Gnome extension.
 * This function will disable and remove extension files.
 * **Restart session will be require to remove the gnome extension.**
*/
export declare function uninstallExtension(): boolean
/**
 * Enable Gnome extensions required for Linux using Gnome > 41.
 * This function will enable extension needed to correctly detect working windows with Wayland desktop environment.
*/
export declare function enableExtension(): boolean
/**
 * Disable Gnome extensions required for Linux using Gnome > 41.
 * This function will disable extension needed to correctly detect working windows with Wayland desktop environment.
*/
export declare function disableExtension(): boolean
/**
 * Return true of false if gnome extension is enabled for Linux using Gnome > 41.
 * This function will return true or false if the extension is set to enabled on extension info. Working only with Wayland windows manager.
*/
export declare function isEnabledExtension(): boolean
/**
 * Return true of false the extensions is installed for Linux using Gnome > 41.
 * This function will return true or false if the extension is correctly installed. Working only with Wayland windows manager.
*/
export declare function isInstalledExtension(): boolean
/**
 * Struct to store all informations of the window
*/
export declare class WindowInfo {
  id: number
  os: string
  title: string
  position: WindowPosition
  info: ProcessInfo
  usage: UsageInfo
  constructor(id: number, os: string, title: string, position: WindowPosition, info: ProcessInfo, usage: UsageInfo)
  /**
  * Funciton who help to recover icon of application and will return `IconInfo`.
  */
  getIcon(): IconInfo
  /**
  * Promise funciton who help to recover icon of application and will return `IconInfo`.
  */
  getIconAsync(): Promise<IconInfo>
  /**
  * Getter to recover browser url
  */
  get url(): string
}
