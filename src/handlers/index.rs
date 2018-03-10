use rocket_contrib::Template;
use diesel::prelude::*;
use diesel::dsl::count;
use chrono::prelude::*;

use metrix::get_player_data;
use models::*;
use schema::posts;
use schema::pages;
use db::DB;
use helpers::*;

// Home many posts gets displayed at once
const POSTS_PER_PAGE: i64 = 5;

#[get("/")]
pub fn index(user: Option<User>, conn: DB) -> Template {
  post_page(user, conn, 1)
}

#[get("/posts/<page_number>")]
pub fn index_page(page_number: i64, user: Option<User>, conn: DB) -> Template {
  post_page(user, conn, page_number)
}

#[get("/post/<post_id>")]
pub fn get_one_post(post_id: i32, user: Option<User>, conn: DB) -> Template {
  let post = posts::table
    .find(post_id)
    .first::<Post>(&*conn)
    .ok();
  let mut context = default_context(conn, user);
  context.add("post", &post);
  Template::render("post", &context)
}

fn post_page(user: Option<User>, conn: DB, page_number: i64) -> Template {
  use schema::posts::dsl::id;
  use schema::posts::dsl::created;

  // Get BagTag data from Metrix API
  let players = get_player_data().unwrap_or(vec![]);

  // Count how many posts we have in total
  let posts_count = posts::table
    .select(count(id))
    .first(&*conn)
    .unwrap_or(0);

  // Get POSTS_PER_PAGE posts
  let posts = posts::table
    .order(created.desc())
    .limit(POSTS_PER_PAGE)
    .offset((page_number - 1) * POSTS_PER_PAGE)
    .load::<Post>(&*conn)
    .unwrap_or(vec![]);

  // Count how many post pages we have in total
  let page_count = (posts_count as f64 / POSTS_PER_PAGE as f64).ceil() as i64;

  let mut context = default_context(conn, user);
  context.add("players", &players);
  context.add("posts", &posts);
  context.add("page_count", &page_count);
  context.add("page_number", &page_number);
  context.add("page_numbers", &page_numbers(page_number, page_count));
  Template::render("index", &context)
}

fn page_numbers(page_number: i64, page_count: i64) -> Vec<i64> {
  match () {
    _ if page_number < 1 => vec![],
    _ if page_count <= 5 => (1..page_count + 1).collect(),
    _ if page_number <= 3 => vec![1,2,3,4,page_count],
    _ if page_number - page_number < 3 => vec![1, page_count - 3, page_count - 2, page_count - 1, page_count],
    _ => vec![1, page_number - 1, page_number, page_number + 1, page_count]
  }
}