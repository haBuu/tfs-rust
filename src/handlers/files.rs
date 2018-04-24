use rocket::response::NamedFile;

use std::path::{Path, PathBuf};

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/<file..>")]
fn images(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("images/").join(file)).ok()
}