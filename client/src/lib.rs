#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;

use stdweb::js_export;

#[js_export]
pub fn hash() -> String {
  format!("hi")
}

pub fn render() -> String {
  format!("<h1>Isomorphic-ish?.</h1>")
}
