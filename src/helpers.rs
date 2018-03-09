use tera::{Result, Value, to_value, try_get_value, Context};
use pulldown_cmark::{html, Parser};
use std::collections::HashMap;

use db::DB;
use models::User;
use models::page::get_top_level_pages;

// What should be available by default in the template context
pub fn default_context(conn: DB, user: Option<User>) -> Context {
  let top_level_pages = get_top_level_pages(&conn);
  let mut context = Context::new();
  context.add("user", &user);
  context.add("top_level_pages", &top_level_pages);
  return context;
}

// Tera filter to convert Markdown to HTML
pub fn markdown_filter(value: Value, _: HashMap<String, Value>) -> Result<Value> {
  let s = try_get_value!("markdown", "value", String, value);
  let parser = Parser::new(s.as_str());
  let mut html_buf = String::new();
  html::push_html(&mut html_buf, parser);
  Ok(to_value(html_buf).unwrap())
}