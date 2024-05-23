import test from 'ava';
import os from 'os';
import { activeWindow, activeWindowAsync, openWindows, openWindowsAsync, subscribeActiveWindow, unsubscribeActiveWindow, unsubscribeAllActiveWindow } from '../index.js';

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

test('activeWindow', (t) => {
  console.time('activeWindow');
  const data = activeWindow();
  console.timeEnd('activeWindow');
  t.log(data);
  compareStruct(t, data);
  t.pass();
})

test('openWindows', (t) => {
  console.time('openwindows');
  const list = openWindows();
  console.timeEnd('openwindows');
  t.log(list);
  t.not(list.length, 0);
  for (const data of list) {
    compareStruct(t, data);
  }
  t.pass();
})

test('subscribeActiveWindow', async (t) => {
  try {
    const data1 = await new Promise((resolve, reject) => {
      console.time('subscribeActiveWindow1');
      const r = subscribeActiveWindow((info) => {
        console.timeEnd('subscribeActiveWindow1');
        t.log(r, info);
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
        t.log(r, info);
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
        t.log(r, info);
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
    t.pass();
  } catch (error) {
    unsubscribeAllActiveWindow();
    throw error;
  }
})


test('unsubscribeAllActiveWindow', async (t) => {
  try {
    const data1 = await new Promise((resolve, reject) => {
      const r = subscribeActiveWindow((info) => {
        t.log(r, info);
        if (info?.id) {
          resolve(info);
        } else {
          reject(new Error('Test failed! no id for active window!'));
        }
      });
    });

    const data2 = await new Promise((resolve, reject) => {
      const r = subscribeActiveWindow((info) => {
        t.log(r, info);
        if (info?.id) {
          resolve(info);
        } else {
          reject(new Error('Test failed! no id for active window!'));
        }
      });
    });

    const data3 = await new Promise((resolve, reject) => {
      const r = subscribeActiveWindow((info) => {
        t.log(r, info);
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
    t.pass();
  } catch (error) {
    unsubscribeAllActiveWindow();
    throw error;
  }
})

test('activeWindowAsync', async (t) => {
  console.time('activeWindowAsync');
  const data = await activeWindowAsync();
  console.timeEnd('activeWindowAsync');
  t.log(data);
  compareStruct(t, data);
  t.pass();
})

test('openWindowsAsync', async (t) => {
  console.time('openWindowsAsync');
  const list = await openWindowsAsync();
  console.timeEnd('openWindowsAsync');
  t.log(list);
  t.not(list.length, 0);
  for (const data of list) {
    compareStruct(t, data);
  }
  t.pass();
})
