#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod component;

#[wasm_bindgen]
pub fn add() -> String {
  component::render("Sup")
}

pub fn render() -> String {
  component::render("Sup")
}
