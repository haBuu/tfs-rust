#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(decl_macro)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
extern crate r2d2_diesel;

#[macro_use] extern crate serde_derive;

extern crate tera;
extern crate pulldown_cmark;
extern crate chrono;

extern crate argon2rs;
extern crate rand;

use rocket_contrib::Template;
use rocket::{Route, Request};
use rocket::response::NamedFile;

use std::path::{Path, PathBuf};
use std::io::stdout;

pub mod db;
pub mod schema;
pub mod models;
pub mod metrix;
pub mod handlers;
pub mod helpers;

use handlers::*;
use helpers::*;

embed_migrations!("migrations");

#[catch(404)]
fn not_found(req: &Request) -> Template {
  let mut map = std::collections::HashMap::new();
  map.insert("path", req.uri().as_str());
  Template::render("404", &map)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/<file..>")]
fn images(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("images/").join(file)).ok()
}

pub fn routes() -> Vec<Route> {
  routes![
    // Public routes
    index, page, index_page, get_one_post
    // Admin routes
    , admin, login, login_user, logout, admin_no_user
    , get_users, new_user, get_user, edit_user, remove_user
    , new_post, add_post, posts, get_post, remove_post, edit_post
    , get_pages, new_page, get_page_with_content, get_page_no_content
    , new_content, remove_page, restore_page, edit_page
    , upload_image
  ]
}

fn rocket() -> rocket::Rocket {
  let conn = db::init_pool();
  let handle = conn.get().expect("Couldn't get a database handle");
  embedded_migrations::run_with_output(&*handle, &mut stdout()).unwrap();
  rocket::ignite()
    // Normal routes
    .mount("/", routes())
    // Static files
    .mount("/files", routes! [files])
    // Images
    .mount("/images", routes! [images])
    .attach(Template::custom(|context| {
      context.tera.register_filter("markdown", markdown_filter);
    }))
    .manage(conn)
    .catch(catchers![not_found])
}

fn main() {
  rocket().launch();
}