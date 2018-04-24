use rocket::{Request, State, Outcome};
use rocket::request::{self, FromRequest};

use tera::{Result, Value, to_value, try_get_value, Context};
use pulldown_cmark::{html, Parser, OPTION_ENABLE_TABLES};
use std::collections::HashMap;

use db::DB;
use models::User;
use models::page::get_top_level_pages;
use Config;

pub struct DefaultContext(pub Context);

impl<'a, 'r> FromRequest<'a, 'r> for DefaultContext {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<DefaultContext, ()> {
    let conn = request.guard::<DB>()?;
    let user = request.guard::<Option<User>>()?;
    let cfg = request.guard::<State<Config>>()?;
    Outcome::Success(DefaultContext(default_context(conn, user, cfg.analytics.to_owned())))
  }

}

// What should be available by default in the template context
pub fn default_context(conn: DB, user: Option<User>, analytics: String) -> Context {
  let top_level_pages = get_top_level_pages(&conn);
  let mut context = Context::new();
  context.add("user", &user);
  context.add("top_level_pages", &top_level_pages);
  context.add("analytics", &analytics);
  return context;
}

// Tera filter to convert Markdown to HTML
pub fn markdown_filter(value: Value, _: HashMap<String, Value>) -> Result<Value> {
  let s = try_get_value!("markdown", "value", String, value);
  let parser = Parser::new_ext(s.as_str(), OPTION_ENABLE_TABLES);
  let mut html_buf = String::new();
  html::push_html(&mut html_buf, parser);
  Ok(to_value(html_buf).unwrap())
}