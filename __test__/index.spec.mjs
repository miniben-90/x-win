import test from 'ava';
import os from 'os';
import { activeWindow, activeWindowAsync, openWindows, openWindowsAsync, subscribeActiveWindow, unsubscribeActiveWindow, unsubscribeAllActiveWindow } from '../index.js';
import { exec } from 'node:child_process';

const Browsers = [
  "msedge",
  "Safari"
]

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

function isWinOrDarwinOs() {
  return ['darwin', 'win32'].findIndex(t => t === os.platform()) !== -1;
}

async function runBrowserToTest() {
  if (os.platform() === 'win32') {
    exec('start microsoft-edge:https://github.com --no-first-run --restore-last-session');
  } else if (os.platform() === 'darwin') {
    exec('open -a Safari https://github.com');
  }
  await sleep(2000);
}

async function killBrowserToTest() {
  if (os.platform() === 'win32') {
    exec('taskkill /f /im msedge.exe');
  } else if (os.platform() === 'darwin') {
    exec('killall Safari');
  }
  await sleep(2000);
}

test.before.skip(async (t) => {
  if (isWinOrDarwinOs()) {
    await runBrowserToTest();
  }
})

const defaultStruct = {
  os: os.platform(),
  info: { execName: "", name: "", path: "", processId: 0 },
  position: { height: 0, width: 0, x: 0, y: 0, isFullScreen: false },
  processId: 0,
  title: "",
  usage: { memory: 0 },
};

/**
 * Compare struct 
 * @param {*} t 
 * @param {*} data 
 */
function compareStruct(t, data) {
  const defaultkeys = Object.entries(defaultStruct);
  for (const [key, value] of defaultkeys) {
    /** For darwin with permission issue should ignore title it will be empty */
    if (os.platform() === 'darwin' && key === 'title') {
      continue;
    }
    if (!(key === 'title' && data.os === 'win32' && data.info.execName === 'Widgets')) {
      if (key === 'os') {
        t.deepEqual(value, data[key]);
      } else {
        t.notDeepEqual(value, data[key]);
      }
    }
  }
}

function compareIconStruct(t, data) {
  t.notDeepEqual(data.data, "");
  t.notDeepEqual(data.width, 0);
  t.notDeepEqual(data.height, 0);
}

test('activeWindow', (t) => {
  console.time('activeWindow');
  const data = activeWindow();
  console.timeEnd('activeWindow');
  compareStruct(t, data);
  return t.pass();
})

test('openWindows', (t) => {
  console.time('openwindows');
  const list = openWindows();
  console.timeEnd('openwindows');
  t.not(list.length, 0);
  for (const data of list) {
    compareStruct(t, data);
  }
  return t.pass();
})

test('subscribeActiveWindow', async (t) => {
  try {
    const data1 = await new Promise((resolve, reject) => {
      console.time('subscribeActiveWindow1');
      const r = subscribeActiveWindow((info) => {
        console.timeEnd('subscribeActiveWindow1');
        if (info?.id) {
          unsubscribeActiveWindow(r);
          resolve(info);
        } else {
          reject(new Error('Test failed! no id for active window!'));
        }
      });
    });

    const data2 = await new Promise((resolve, reject) => {
      console.time('subscribeActiveWindow2');
      const r = subscribeActiveWindow((info) => {
        console.timeEnd('subscribeActiveWindow2');
        if (info?.id) {
          unsubscribeActiveWindow(r);
          resolve(info);
        } else {
          reject(new Error('Test failed! no id for active window!'));
        }
      });
    });

    const data3 = await new Promise((resolve, reject) => {
      console.time('subscribeActiveWindow3');
      const r = subscribeActiveWindow((info) => {
        console.timeEnd('subscribeActiveWindow3');
        if (info?.id) {
          unsubscribeActiveWindow(r);
          resolve(info);
        } else {
          reject(new Error('Test failed! no id for active window!'));
        }
      });
    });
    compareStruct(t, data1);
    compareStruct(t, data2);
    compareStruct(t, data3);
    return t.pass();
  } catch (error) {
    unsubscribeAllActiveWindow();
    throw error;
  }
})


test('unsubscribeAllActiveWindow', async (t) => {
  try {
    const data1 = await new Promise((resolve, reject) => {
      const r = subscribeActiveWindow((info) => {
        if (info?.id) {
          resolve(info);
        } else {
          reject(new Error('Test failed! no id for active window!'));
        }
      });
    });

    const data2 = await new Promise((resolve, reject) => {
      const r = subscribeActiveWindow((info) => {
        if (info?.id) {
          resolve(info);
        } else {
          reject(new Error('Test failed! no id for active window!'));
        }
      });
    });

    const data3 = await new Promise((resolve, reject) => {
      const r = subscribeActiveWindow((info) => {
        if (info?.id) {
          resolve(info);
        } else {
          reject(new Error('Test failed! no id for active window!'));
        }
      });
    });
    compareStruct(t, data1);
    compareStruct(t, data2);
    compareStruct(t, data3);
    unsubscribeAllActiveWindow();
    return t.pass();
  } catch (error) {
    unsubscribeAllActiveWindow();
    throw error;
  }
})

test('activeWindowAsync', async (t) => {
  console.time('activeWindowAsync');
  const data = await activeWindowAsync();
  console.timeEnd('activeWindowAsync');
  compareStruct(t, data);
  return t.pass();
})

test('openWindowsAsync', async (t) => {
  console.time('openWindowsAsync');
  const list = await openWindowsAsync();
  console.timeEnd('openWindowsAsync');
  for (const data of list) {
    compareStruct(t, data);
  }
  return t.pass();
})

test('getIcon', (t) => {
  const data = activeWindow();
  console.time('getIcon');
  const iconInfo = data.getIcon();
  console.timeEnd('getIcon');
  compareIconStruct(t, iconInfo);
  return t.pass();
})

test('getIconAsync', async (t) => {
  const data = activeWindow();
  console.time('getIconAsync');
  const iconInfo = await data.getIconAsync();
  console.timeEnd('getIconAsync');
  compareIconStruct(t, iconInfo);
  return t.pass();
})

if (os.platform() === 'win32' || os.platform() === 'darwin') {

  test.skip('url getter - activeWindow', (t) => {
    console.time('activeWindow');
    const data = activeWindow();
    console.timeEnd('activeWindow');
    t.not(data.url.startsWith('http'));
    return t.pass();
  })

  test.skip('url getter - activeWindowAsync', async (t) => {
    console.time('url getter - activeWindowAsync');
    const data = await activeWindowAsync();
    console.timeEnd('url getter - activeWindowAsync');
    t.not(data.url.startsWith('http'));
    return t.pass();
  })

  test.skip('url getter - openWindows', (t) => {
    console.time('openwindows');
    const list = openWindows();
    console.timeEnd('openwindows');
    t.not(list.length, 0);
    const filtred = list.filter(window_info => Browsers.findIndex(browser => {
      t.log(window_info.info.execName);
      return browser === window_info.info.execName && window_info.url.startsWith('http')
    }) !== -1);
    t.not(filtred.length, 0);
    return t.pass();
  })
}

test.after.always.skip(async () => {
  if (isWinOrDarwinOs()) {
    await killBrowserToTest();
  }
})