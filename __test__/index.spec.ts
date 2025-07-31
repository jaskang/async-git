import test from 'ava'

import { gitStatus, gitStatusWithFetch } from '../index.js'

const modules: string[] = []
test('sync function from native code', async (t) => {
  const dirs = modules.map((m) => `/Users/jaskang/Documents/codes/-miniapp/${m.path}`)
  const start = Date.now()
  await gitStatus(dirs)
    .then((results) => {
      console.log('lines:', results)
      const end = Date.now()
      console.log(`gitStatusWithFetch took ${end - start} ms`)
    })
    .catch((error) => {
      console.error('error:', error)
      t.fail('Expected the command to succeed, but it failed.')
    })
})
