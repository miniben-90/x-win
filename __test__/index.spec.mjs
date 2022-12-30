import test from 'ava'
import os from 'os'
import { activeWindow, openWindows } from '../index.js'

const defaultStruct = {
  os: os.platform(),
  info: { execName: "", name: "", path: "", processId: 0 },
  position: { height: 0, width: 0, x: 0, y: 0 },
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
})

test('openWindows', (t) => {
  console.time('openwindows');
  const list = openWindows();
  console.timeEnd('openwindows');
  t.log(list);
  for (const data of list) {
    compareStruct(t, data);
  }
})

