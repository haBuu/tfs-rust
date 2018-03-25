pub mod page;
pub use self::page::*;

use chrono::prelude::*;
use schema::{posts, pages, users, contents};

#[derive(Identifiable, Queryable, Associations, Serialize, Debug)]
#[belongs_to(User)]
#[table_name = "pages"]
pub struct Page {
  pub id: i32,
  pub name: String,
  pub user_id: i32,
  pub rank: i32,
  pub top_level: bool,
  pub removed: bool,
  pub created: NaiveDateTime,
  pub modified: Option<NaiveDateTime>
}

#[derive(Identifiable, Queryable, Serialize, Debug)]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub name: String,
  pub password: String,
  pub admin: bool,
  pub superadmin: bool,
  pub created: NaiveDateTime
}

#[derive(Identifiable, Queryable, Associations, Serialize, Debug)]
#[belongs_to(User)]
#[table_name = "posts"]
pub struct Post {
  pub id: i32,
  pub created: NaiveDateTime,
  pub user_id: i32,
  pub modified: Option<NaiveDateTime>,
  pub user_modified_id: Option<i32>,
  pub title: String,
  pub content: String
}

#[derive(Identifiable, Queryable, Associations, Serialize, Debug)]
#[belongs_to(Page, User)]
#[table_name = "contents"]
pub struct Content {
  pub id: i32,
  pub page_id: i32,
  pub user_id: i32,
  pub content: String,
  pub version: i32,
  pub created: NaiveDateTime,
  pub comment: Option<String>
}