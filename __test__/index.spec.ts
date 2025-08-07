import test from 'ava'

import { gitStatus, gitStatusWithFetch } from '../index.js'

const modules: any[] = []
test('sync function from native code', async (t) => {
  const dirs = modules.map((m) => `/Users/jaskang/Documents/codes/test/${m.path}`)
  const start = Date.now()
  await gitStatusWithFetch(dirs)
    .then((results) => {
      console.log('lines:', results)
      const end = Date.now()
      console.log(`gitStatusWithFetch took ${end - start} ms`)
    })
    .catch((error) => {
      console.error('error:', error)
      t.fail('Expected the command to succeed, but it failed.')
    })
  t.is(true, true, 'This test is a placeholder and should be replaced with actual assertions.')
})
