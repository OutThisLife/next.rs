extern crate actix;
extern crate actix_web;
extern crate client;
extern crate env_logger;

use self::client::render;
use actix_web::{error, fs::NamedFile, Error, HttpRequest, HttpResponse, Query, State};
use std::collections::HashMap;
use std::path::PathBuf;
use tera::{Context, Tera};

pub struct AppState {
  pub templates: Tera,
}

pub fn index(
  (state, _): (State<AppState>, Query<HashMap<String, String>>),
) -> Result<HttpResponse, Error> {
  let mut ctx = Context::new();
  ctx.add("yield", &render());

  let s = state
    .templates
    .render("index.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))
    .unwrap();

  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub fn files(req: HttpRequest<AppState>) -> actix_web::Result<NamedFile> {
  let file: PathBuf = req.match_info().query("tail")?;
  let path: String = ["./../dist/", &file.into_os_string().into_string().unwrap()].join("");
  Ok(NamedFile::open(path)?)
}
