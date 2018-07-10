extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate tera;

mod routes;

use actix::System;
use actix_web::{http, middleware, server, App};

const URL: &str = "[::1]:8000";

fn main() {
  ::std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let sys = System::new("nextrs");

  let _server = server::new(move || {
    let mut tera = compile_templates!("./../dist/**/*.html");
    tera.autoescape_on(vec![]);

    App::with_state(routes::AppState { templates: tera })
      .middleware(middleware::Logger::default())
      .resource("/", |r| r.method(http::Method::GET).with(routes::index))
      .resource(r"/{tail:.*}", |r| r.f(routes::files))
  }).bind(URL)
    .unwrap()
    .start();

  println!("Started http server: {}", URL);

  let _ = sys.run();
}
