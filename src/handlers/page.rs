use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;
use diesel::prelude::*;
use diesel::{insert_into, update};

use db::DB;
use models::User;
use models::Page;
use models::Content;
use schema::pages;
use handlers::admin::Admin;
use models::page::*;

use helpers::*;

#[derive(AsChangeset, FromForm, Debug)]
#[table_name = "pages"]
pub struct PageForm {
  name: String,
  top_level: bool,
  rank: i32,
}

#[get("/<page_name>", rank = 3)]
pub fn page(page_name: String, user: Option<User>, conn: DB) -> Template {
  let page_content = get_page_content_by_name(&conn, page_name);
  let mut context = default_context(conn, user);
  context.add("page_content", &page_content);
  Template::render("page", &context)
}

#[get("/admin/pages")]
pub fn get_pages(user: Admin, conn: DB) -> Template {
  let other_pages = get_other_pages(&conn);
  let removed_pages = get_removed_pages(&conn);
  let mut context = default_context(conn, Some(user.0));
  context.add("other_pages", &other_pages);
  context.add("removed_pages", &removed_pages);
  Template::render("pages", &context)
}

fn get_page(page: i32, content_id: Option<i32>, user: Admin, conn: DB) -> Template {
  use schema::pages::dsl::pages;
  use schema::contents::dsl::contents;
  use schema::contents::dsl::version;
  use schema::contents::dsl::page_id;

  let this_page = pages.find(page).first::<Page>(&*conn).expect("Error finding page");

  let content = match content_id {
    Some(id) => contents.find(id).first::<Content>(&*conn).ok(),
    None => contents.filter(page_id.eq(page)).order(version.desc()).first::<Content>(&*conn).ok()
  };

  let versions = contents.filter(page_id.eq(page))
    .order(version.desc())
    .load::<Content>(&*conn)
    .unwrap_or(vec![]);

  let mut context = default_context(conn, Some(user.0));
  context.add("page", &this_page);
  context.add("content", &content);
  context.add("versions", &versions);
  Template::render("page-content", &context)
}

#[get("/admin/page/<page>/<content_id>")]
pub fn get_page_with_content(page: i32, content_id: i32, user: Admin, conn: DB) -> Template {
  get_page(page, Some(content_id), user, conn)
}

#[get("/admin/page/<page>")]
pub fn get_page_no_content(page: i32, user: Admin, conn: DB) -> Template {
  get_page(page, None, user, conn)
}

#[post("/admin/page", data = "<form>")]
pub fn new_page(user: Admin, conn: DB, form: Form<PageForm>) -> Redirect {
  use schema::pages::dsl::*;
  let page_form = form.get();
  insert_into(pages)
    .values((
      name.eq(&page_form.name),
      user_id.eq(user.0.id),
      top_level.eq(&page_form.top_level),
      rank.eq(&page_form.rank),
    ))
    .execute(&*conn)
    .expect("Error adding new page");
  Redirect::to("/admin/pages")
}

#[post("/admin/page/<page_id>", data = "<form>")]
pub fn edit_page(page_id: i32, _user: Admin, conn: DB, form: Form<PageForm>) -> Redirect {
  use schema::pages::dsl::*;
  let page_form = form.get();
  update(pages.find(page_id))
    .set(page_form)
    .execute(&*conn)
    .expect("Error updating page");
  Redirect::to("/admin/pages")
}

#[derive(FromForm, Debug)]
pub struct ContentForm {
  content: String,
  comment: Option<String>,
}

#[post("/admin/page/content/<page>", data = "<form>")]
pub fn new_content(page: i32, user: Admin, conn: DB, form: Form<ContentForm>) -> Redirect {
  use schema::contents::dsl::*;

  let content_form = form.get();

  let current_version = contents
    .filter(page_id.eq(page))
    .select(version)
    .order(version.desc())
    .first::<i32>(&*conn)
    .unwrap_or(0);

  insert_into(contents)
    .values((
      page_id.eq(&page),
      user_id.eq(user.0.id),
      content.eq(&content_form.content),
      comment.eq(&content_form.comment),
      version.eq(current_version + 1),
    ))
    .execute(&*conn)
    .expect("Error adding page content");

  Redirect::to("/admin/pages")
}

#[post("/admin/page/remove/<page_id>")]
pub fn remove_page(page_id: i32, _user: Admin, conn: DB) -> Redirect {
  use schema::pages::dsl::*;

  update(pages.find(page_id))
    .set(removed.eq(true))
    .execute(&*conn)
    .expect("Error removing page");

  Redirect::to("/admin/pages")
}

#[post("/admin/page/restore/<page_id>")]
pub fn restore_page(page_id: i32, _user: Admin, conn: DB) -> Redirect {
  use schema::pages::dsl::*;

  update(pages.find(page_id))
    .set(removed.eq(false))
    .execute(&*conn)
    .expect("Error restoring page");

  Redirect::to("/admin/pages")
}