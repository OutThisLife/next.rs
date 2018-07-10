extern crate actix;
extern crate actix_web;
extern crate client;
extern crate env_logger;
extern crate tera;

use actix::System;
use actix_web::{error, http, middleware, server, App, Error, HttpResponse, Query, State};
use client::render;
use std::collections::HashMap;
use tera::{Context, Tera};

struct AppState {
  template: tera::Tera,
}

fn index(
  (state, _): (State<AppState>, Query<HashMap<String, String>>),
) -> Result<HttpResponse, Error> {
  let mut ctx = Context::new();
  ctx.add("yield", &render());

  let s = state
    .template
    .render("index.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))
    .unwrap();

  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// -----------------------------------------------------

const URL: &str = "[::1]:8000";

fn main() {
  ::std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let sys = System::new("nextrs");

  let _server = server::new(move || {
    App::with_state(AppState {
      template: Tera::new("./../static/**/*.html").unwrap(),
    }).middleware(middleware::Logger::default())
      .resource("/", |r| r.method(http::Method::GET).with(index))
  }).bind(URL)
    .unwrap()
    .start();

  println!("Started http server: {}", URL);

  let _ = sys.run();
}

#[test]
fn test_static_load() {
  let result = || -> tera::Result<String> {
    let tera = Tera::new("./../static/**/*").unwrap();
    let mut ctx = Context::new();
    ctx.add("yield", &"hi");

    tera.render("index.html", &Context::new())
  }();

  assert_eq!(result.is_err(), true);
  let errs = result.unwrap_err();
  assert_eq!(
    errs.iter().nth(0).unwrap().description(),
    "Failed to render"
  );
}
