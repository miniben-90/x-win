import { Bench } from 'tinybench'
import { activeWindow, activeWindowAsync } from '../index.js'
import { activeWindow as activeWindowwOld, activeWindowAsync as activeWindowAsyncOld } from '@miniben90/x-win'
import { activeWindow as getactiveWindow, activeWindowSync as getactiveWindowSync } from 'get-windows'

const benchA = new Bench({
  name: 'Benchmark between repo version, previous version and latest get-windows',
})

benchA.concurrency = 'task'

const getWindowVersion = '9.2.3'

benchA
  .add('current workspace - activeWindow', () => {
    activeWindow()
  })
  .add('current workspace - activeWindowAsync', async () => {
    await activeWindowAsync()
  })
  .add('@miniben90/x-win:3.1.0 - activeWindow', () => {
    activeWindowwOld()
  })
  .add('@miniben90/x-win:3.1.0 - activeWindowAsync', async () => {
    await activeWindowAsyncOld()
  })
  .add(`get-windows:${getWindowVersion} - activeWindowSync`, async () => {
    getactiveWindowSync()
  })
  .add(`get-windows:${getWindowVersion} - activeWindow`, async () => {
    await getactiveWindow()
  })

await benchA.run()
console.table(benchA.table())
