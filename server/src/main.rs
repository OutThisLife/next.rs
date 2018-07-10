extern crate actix;
extern crate actix_web;
extern crate env_logger;

use actix::System;
use actix_web::{fs, middleware, server, App};

const URL: &str = "[::1]:8000";

fn main() {
  ::std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let sys = System::new("nextrs");

  let _server = server::new(move || {
    App::new()
      .middleware(middleware::Logger::default())
      .handler(
        "/",
        fs::StaticFiles::new("./../static/").index_file("index.html"),
      )
  }).bind(URL)
    .unwrap()
    .start();

  println!("Started http server: {}", URL);

  let _ = sys.run();
}
