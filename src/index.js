import { add, wasmBooted } from './client/src/lib.rs'

wasmBooted.then(f => {
  console.log('sup', add(5))
})
