extern crate actix;
extern crate actix_web;

use actix::System;
use actix_web::{server, App, Error, HttpRequest, HttpResponse};

fn index(_req: HttpRequest) -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .content_type("text/html")
      .body("Hellof world"),
  )
}

fn main() {
  const URL: &str = "[::1]:8000";
  let sys = System::new("nextrs");

  let _server = server::new(|| App::new().handler("/", index))
    .bind(URL)
    .unwrap()
    .start();

  println!("Started http server: {}", URL);
  let _ = sys.run();
}
