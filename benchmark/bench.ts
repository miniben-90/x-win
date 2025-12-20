import { Bench } from 'tinybench'
import { activeWindow, activeWindowAsync } from '../index.js'
import { activeWindow as activeWindowwOld, activeWindowAsync as activeWindowAsyncOld } from '@miniben90/x-win'
import { activeWindow as getactiveWindow, activeWindowSync as getactiveWindowSync, MacOSResult } from 'get-windows'
import { readFileSync } from 'fs'
import { resolve, dirname } from 'path'
import { fileURLToPath } from 'url'

const benchmarkActiveWindow: Bench = new Bench({
  name: 'Benchmark between local repo, previous version of x-win and latest version of get-windows',
  concurrency: 'task',
  iterations: 1000,
  setup: (_task, mode) => {
    if (mode === 'warmup' && typeof globalThis.gc === 'function') {
      globalThis.gc()
    }
  },
  time: 100,
})

const __dirname = dirname(fileURLToPath(import.meta.url))
const node_path = resolve(__dirname, 'node_modules')
const getWindowVersion = JSON.parse(readFileSync(resolve(node_path, 'get-windows', 'package.json'), 'utf-8')).version
const xWinVersion = JSON.parse(readFileSync(resolve(node_path, '@miniben90', 'x-win', 'package.json'), 'utf-8')).version

benchmarkActiveWindow
  .add('current workspace - activeWindow', () => {
    activeWindow()
  })
  .add(`@miniben90/x-win:${xWinVersion} - activeWindow`, () => {
    activeWindowwOld()
  })
  .add(`get-windows:${getWindowVersion} - activeWindowSync`, () => {
    getactiveWindowSync()
  })
  .add('current workspace - activeWindowAsync', async () => {
    await activeWindowAsync()
  })
  .add(`@miniben90/x-win:${xWinVersion} - activeWindowAsync`, async () => {
    await activeWindowAsyncOld()
  })
  .add(`get-windows:${getWindowVersion} - activeWindow`, async () => {
    await getactiveWindow()
  })

await benchmarkActiveWindow.run()
const table = benchmarkActiveWindow.table()
console.table(table)

const benchmarkBrowserUrl: Bench = new Bench({
  name: 'Benchmark between local repo, previous version of x-win and latest version of get-windows url recovery',
  concurrency: 'task',
  iterations: 1000,
  setup: (_task, mode) => {
    if (mode === 'warmup' && typeof globalThis.gc === 'function') {
      globalThis.gc()
    }
  },
  time: 100,
})

benchmarkBrowserUrl
  .add('current workspace - activeWindow.url', () => {
    activeWindow().url
  })
  .add(`@miniben90/x-win:${xWinVersion} - activeWindow.url`, () => {
    activeWindowwOld().url
  })
  .add(`get-windows:${getWindowVersion} - activeWindowSync.url`, () => {
    ;(getactiveWindowSync() as MacOSResult)?.url
  })
  .add('current workspace - activeWindowAsync.url', async () => {
    ;(await activeWindowAsync()).url
  })
  .add(`@miniben90/x-win:${xWinVersion} - activeWindowAsync.url`, async () => {
    ;(await activeWindowAsyncOld()).url
  })
  .add(`get-windows:${getWindowVersion} - activeWindow.url`, async () => {
    ;((await getactiveWindow()) as MacOSResult)?.url
  })

await benchmarkBrowserUrl.run()
const tableUrl = benchmarkBrowserUrl.table()
console.table(tableUrl)
