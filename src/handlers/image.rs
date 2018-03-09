use rocket::Data;
use std::io;

// TODO: error handling
#[post("/admin/image", format = "image/*", data = "<data>")]
fn upload_image(data: Data) -> io::Result<String> {
  data.stream_to_file("images/upload.dat")?;
  Ok("/images/upload.dat".to_string())
}