import { client_render as render } from './../client/Cargo.toml'

try {
  render()
} catch (e) {
  console.error(e)
}
