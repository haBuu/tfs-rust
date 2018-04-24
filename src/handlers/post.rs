use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;
use diesel::prelude::*;
use diesel::{insert_into, delete, update};

use db::DB;
use models::Post;
use schema::posts;
use handlers::admin::Admin;
use helpers::*;

#[derive(FromForm, Debug)]
pub struct PostForm {
  title: String,
  content: String,
}

#[get("/admin/posts")]
pub fn posts(_user: Admin, conn: DB, ctx: DefaultContext) -> Template {
  use schema::posts::dsl::created;
  let posts = posts::table
    .order(created.desc())
    .load::<Post>(&*conn)
    .unwrap_or(vec![]);
  let mut context = ctx.0;
  context.add("posts", &posts);
  Template::render("posts", &context)
}

#[get("/admin/post/add")]
pub fn add_post(_user: Admin, ctx: DefaultContext) -> Template {
  Template::render("add-post", &ctx.0)
}

#[post("/admin/post/add", data = "<form>")]
pub fn new_post(user: Admin, conn: DB, form: Form<PostForm>) -> Redirect {
  use schema::posts::dsl::*;
  let post_form = form.get();
  insert_into(posts)
    .values((
      user_id.eq(user.0.id),
      title.eq(&post_form.title),
      content.eq(&post_form.content),
    ))
    .execute(&*conn)
    .expect("Error adding new post");
  Redirect::to("/admin/posts")
}

#[get("/admin/post/<post_id>")]
pub fn get_post(post_id: i32, _user: Admin, conn: DB, ctx: DefaultContext) -> Template {
  let post = posts::table
    .find(post_id)
    .first::<Post>(&*conn)
    .expect("Error finding post");
  let mut context = ctx.0;
  context.add("post", &post);
  Template::render("edit-post", &context)
}

#[post("/admin/post/remove/<post_id>")]
pub fn remove_post(post_id: i32, _user: Admin, conn: DB) -> Redirect {
  use schema::posts::dsl::*;
  delete(posts.filter(id.eq(post_id))).execute(&*conn).expect("Error deleting post");
  Redirect::to("/admin/posts")
}

#[post("/admin/post/<post_id>", data = "<form>")]
pub fn edit_post(post_id: i32, user: Admin, conn: DB, form: Form<PostForm>) -> Redirect {
  use schema::posts::dsl::*;
  let post_form = form.get();
  update(posts.find(post_id))
    .set((
      title.eq(&post_form.title),
      content.eq(&post_form.content),
      user_modified_id.eq(&user.0.id),
    ))
    .execute(&*conn)
    .expect("Error updating post");
  Redirect::to("/admin/posts")
}