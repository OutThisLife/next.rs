import('./../client/src/lib.rs').then(({ wasmBooted, ...d }) => {
  wasmBooted
    .then(() => {
      let res = d.ok()
      console.log(res)
    })
    .catch(e => {
      console.error(e)
    })
})
