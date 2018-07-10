import client from './../client/Cargo.toml'

try {
  const $root = document.getElementById('root')
  $root.innerHTML = client.client_render()
} catch (e) {
  console.error(e)
}
