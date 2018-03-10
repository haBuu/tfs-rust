use rocket::{State, Outcome};
use rocket::request::{self, Form, FromRequest, Request};
use rocket_contrib::Template;
use rocket::http::{Cookie, Cookies, Status};
use rocket::response::Redirect;
use rocket::outcome::IntoOutcome;

use diesel::prelude::*;

use argon2rs::verifier::Encoded;

use models::User;
use schema::users::dsl::users;
use helpers::*;

use db::Pool;
use db::DB;


pub fn find_user(conn: DB, id: i32) -> Option<User> {
  users.find(id).first::<User>(&*conn).ok()
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {

    let pool = request.guard::<State<Pool>>()?;
    let conn = match pool.get() {
      Ok(c) => DB(c),
      Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ()))
    };

    request.cookies()
      .get_private("user_id")
      .and_then(|cookie| cookie.value().parse::<i32>().ok())
      .and_then(|id| find_user(conn, id))
      .or_forward(())
  }
}

#[derive(Debug)]
pub struct Admin(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, ()> {

    let pool = request.guard::<State<Pool>>()?;
    let conn = match pool.get() {
      Ok(c) => DB(c),
      Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ()))
    };

    request.cookies()
      .get_private("user_id")
      .and_then(|cookie| cookie.value().parse::<i32>().ok())
      .and_then(|id| find_user(conn, id))
      .and_then(|user| if user.admin || user.superadmin { Some(Admin(user)) } else { None })
      .or_forward(())
  }
}

#[derive(Debug)]
pub struct SuperAdmin(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for SuperAdmin {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<SuperAdmin, ()> {

    let pool = request.guard::<State<Pool>>()?;
    let conn = match pool.get() {
      Ok(c) => DB(c),
      Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ()))
    };

    request.cookies()
      .get_private("user_id")
      .and_then(|cookie| cookie.value().parse::<i32>().ok())
      .and_then(|id| find_user(conn, id))
      .and_then(|user| if user.superadmin { Some(SuperAdmin(user)) } else { None })
      .or_forward(())
  }
}

#[get("/admin")]
pub fn admin(user: Admin, conn: DB) -> Template {
  let context = default_context(conn, Some(user.0));
  Template::render("admin", &context)
}

#[get("/admin", rank = 2)]
pub fn admin_no_user() -> Redirect {
  Redirect::to("/login")
}

#[derive(FromForm, Debug)]
pub struct UserLogin {
  username: String,
  password: String
}

#[get("/login")]
pub fn login(conn: DB) -> Template {
  let context = default_context(conn, None);
  Template::render("login", &context)
}

#[post("/login", data = "<user_login>")]
pub fn login_user(mut cookies: Cookies, user_login: Form<UserLogin>, conn: DB) -> Redirect  {
  use schema::users::dsl::*;

  let form = user_login.get();

  // Find user
  let user = match users.filter(name.eq(&form.username)).first::<User>(&*conn) {
    Ok(u) => u,
    Err(_) => return Redirect::to("/login"),
  };

  // Verify password
  let hash = user.password.into_bytes();
  let db_hash = Encoded::from_u8(&hash).expect("Failed to read password hash");
  if !db_hash.verify(form.password.as_ref()) {
    return Redirect::to("/login");
  }

  // Login user
  cookies.add_private(Cookie::new("user_id", user.id.to_string()));
  Redirect::to("/admin")
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
  cookies.remove_private(Cookie::named("user_id"));
  Redirect::to("/login")
}