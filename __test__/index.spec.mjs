import test from 'ava'

import { activeWindow, openWindows } from '../index.js'

const defaultStruct = {
  os: "",
  parentProcess: { execName: "", name: "", path: "", processId: 0 },
  position: { height: 0, width: 0, x: 0, y: 0 },
  processId: 0,
  title: "",
  usage: { memory: 0 },
};

test('activeWindow', (t) => {
  console.time('activeWindow');
  const data = activeWindow();
  console.timeEnd('activeWindow');
  const defaultkeys = Object.entries(defaultStruct);
  // t.log(data);
  for (const [key, value] of defaultkeys) {
    // t.is(data.hasOwnProperty(key), `'${key}' do not exist`);
    t.notDeepEqual(value, data[key]);
  }
})

test('openWindows', (t) => {
  console.time('openwindows');
  const data = openWindows();
  console.timeEnd('openwindows');
  const defaultkeys = Object.entries(defaultStruct);
  // t.log(data);
  for (const [key, value] of defaultkeys) {
    // t.is(data.hasOwnProperty(key), `'${key}' do not exist`);
    t.notDeepEqual(value, data[key]);
  }
})

