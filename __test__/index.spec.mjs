import test from 'ava'

import { isElevated } from '../index.js'

test('native', (t) => {
  t.pass(isElevated(), false)
})
