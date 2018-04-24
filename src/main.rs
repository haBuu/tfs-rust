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
use rocket::fairing::{AdHoc};

use std::io::stdout;

mod db;
mod schema;
mod models;
mod metrix;
mod handlers;
mod helpers;

use handlers::*;
use helpers::*;

embed_migrations!("migrations");

#[catch(404)]
fn not_found(req: &Request) -> Template {
  let mut map = std::collections::HashMap::new();
  map.insert("path", req.uri().as_str());
  Template::render("404", &map)
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

#[derive(Debug)]
pub struct Config {
  // pub analytics: &'static str
  pub analytics: String
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
    .attach(AdHoc::on_attach(|r| {
      let id = r.config().get_str("analytics").unwrap_or("").to_string();
      let cfg = Config { analytics: id };
      Ok(r.manage(cfg))
    }))
}

fn main() {
  rocket().launch();
}

#[cfg(test)]
mod test {
  use super::rocket;
  use rocket::local::Client;
  use rocket::http::Status;
  use handlers::*;

  #[test]
  fn smoke_test() {
    let client = Client::new(rocket()).unwrap();

    // public root
    let mut response = client.get(uri!(index)).dispatch();
    assert_eq!(response.status(), Status::Ok);

    // admin root
    response = client.get(uri!(admin)).dispatch();
    assert_eq!(response.status(), Status::SeeOther);

    // static file style
    response = client.get(uri!(files: "files/css/style.css")).dispatch();
    assert_eq!(response.status(), Status::Ok);

    // static file style not found
    response = client.get(uri!(files: "files/css/style2.css")).dispatch();
    assert_eq!(response.status(), Status::NotFound);
  }
}