#![feature(proc_macro)]

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

#[cfg(target_arch = "wasm32")]
use stdweb::js_export;

fn render_to_html() -> String {
  format!("<h1>Isomorphic-ish?.</h1>")
}

pub fn render() -> String {
  render_to_html()
}

#[cfg(target_arch = "wasm32")]
#[js_export]
fn client_render() -> String {
  render_to_html()
}
