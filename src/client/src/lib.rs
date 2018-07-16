#![feature(
  proc_macro,
  wasm_custom_section,
  wasm_import_module,
  proc_macro_non_items
)]

extern crate maud;
extern crate maud_macros;
extern crate rand;
extern crate wasm_bindgen;

use maud::Markup;
use maud_macros::html;
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[derive(Default)]
#[wasm_bindgen]
pub struct Component {
  name: Option<String>,
  template: Option<Markup>,
}

#[wasm_bindgen]
impl Component {
  pub fn render(&mut self) -> String {
    match self.template.take() {
      Some(tmpl) => tmpl.into_string(),
      None => format!(""),
    }
  }
}

fn with_name<P>(name: &'static str) -> impl Fn(P) -> Component
where
  P: Fn(&str) -> Markup,
{
  move |cb: P| Component {
    name: Some(name.to_string()),
    template: Some(cb(name)),
  }
}

#[wasm_bindgen]
pub fn ok() -> String {
  with_name("web-app")(|name: &str| {
    html!{
      h1 {
        "Hello" (name)
      }

      a href="javascript:;" {
        "baby?"
      }
    }
  }).render()
}
