#![feature(proc_macro)]
#![recursion_limit = "1024"]

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

#[cfg(target_arch = "wasm32")]
use stdweb::js_export;

mod component;

pub fn render() -> String {
  format!("<h1>Isomorphic-ish?.</h1>")
}

#[cfg(target_arch = "wasm32")]
#[js_export]
pub fn client_render() {
  component::render();
}
