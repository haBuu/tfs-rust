use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;
use rocket::http::Status;
use rocket::Data;
use rocket::outcome::IntoOutcome;

use std::io;

#[derive(Debug)]
struct Filename<'a>(&'a str);

impl<'a, 'r> FromRequest<'a, 'r> for Filename<'a> {
  type Error = ();
  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
    request.headers().get_one("filename").map(Filename).or_forward(())
  }
}

#[post("/admin/image", format = "image/*", data = "<data>")]
fn upload_image(data: Data, file: Filename) -> io::Result<String> {
  let path = "images/".to_owned() + file.0;
  data.stream_to_file(&path)?;
  Ok("/".to_owned() + &path)
}