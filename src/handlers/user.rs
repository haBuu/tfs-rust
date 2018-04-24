use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;

use diesel::prelude::*;
use diesel::insert_into;
use diesel::{update, delete};

use argon2rs::verifier::Encoded;
use rand::{self, Rng};

use db::DB;
use models::User;
use schema::users;
use handlers::admin::SuperAdmin;
use helpers::*;

#[get("/admin/users")]
pub fn get_users(_user: SuperAdmin, conn: DB, ctx: DefaultContext) -> Template {
  let users = users::table
    .load::<User>(&*conn)
    .unwrap_or(vec![]);
  let mut context = ctx.0;
  context.add("users", &users);
  Template::render("users", &context)
}

#[get("/admin/user/<user_id>")]
pub fn get_user(user_id: i32, _user: SuperAdmin, conn: DB, ctx: DefaultContext) -> Template {
  let user_edit = users::table
    .find(user_id)
    .first::<User>(&*conn)
    .expect("Error finding user");
  let mut context = ctx.0;
  context.add("user_edit", &user_edit);
  Template::render("user", &context)
}

#[derive(Insertable, FromForm, Debug)]
#[table_name="users"]
pub struct NewUser {
  name: String,
  password: String,
  admin: bool,
  superadmin: bool,
}

#[derive(AsChangeset, FromForm, Debug)]
#[table_name="users"]
pub struct EditUser {
  name: String,
  admin: bool,
  superadmin: bool,
}

#[post("/admin/user/add", data = "<new_user>")]
pub fn new_user(_user: SuperAdmin, conn: DB, new_user: Form<NewUser>) -> Redirect {
  use schema::users::dsl::*;

  let form = new_user.get();

  let user = NewUser {
    name: form.name.to_owned(),
    password: hash_password(&form.password),
    admin: form.admin,
    superadmin: form.superadmin,
  };

  insert_into(users)
    .values(&user)
    .execute(&*conn)
    .expect("Error inserting user");

  Redirect::to("/admin/users")
}

#[post("/admin/user/<user_id>", data = "<form>")]
pub fn edit_user(user_id: i32, _user: SuperAdmin, conn: DB, form: Form<EditUser>) -> Redirect {
  use schema::users::dsl::*;

  update(users.filter(id.eq(user_id)))
    .set(form.get())
    .execute(&*conn)
    .expect("Error updating user");

  Redirect::to("/admin/users")
}

#[post("/admin/user/remove/<user_id>")]
pub fn remove_user(user_id: i32, _user: SuperAdmin, conn: DB) -> Redirect {
  use schema::users::dsl::*;

  delete(users.filter(id.eq(user_id))).execute(&*conn).expect("Error deleting user");

  Redirect::to("/admin/users")
}

fn hash_password(password: &String) -> String {
  let salt = rand::thread_rng().gen::<[u8; 16]>();
  let hash = Encoded::default2i(password.as_bytes(), &salt, &[], &[]).to_u8();
  String::from_utf8(hash.clone()).unwrap()
}