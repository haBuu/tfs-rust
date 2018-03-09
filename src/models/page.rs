use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use schema::pages::dsl::*;
use schema::contents::dsl::*;
use models::Page;
use models::Content;

use diesel::result::Error;

pub fn get_top_level_pages(conn: &MysqlConnection) -> Vec<Page> {
  pages
    .filter(top_level.eq(true))
    .filter(removed.eq(false))
    .order(rank.desc())
    .load::<Page>(conn)
    .unwrap_or(vec![])
}

pub fn get_other_pages(conn: &MysqlConnection) -> Vec<Page> {
  pages
    .filter(top_level.eq(false))
    .filter(removed.eq(false))
    .order(rank.desc())
    .load::<Page>(conn)
    .unwrap_or(vec![])
}

pub fn get_removed_pages(conn: &MysqlConnection) -> Vec<Page> {
  pages
    .filter(removed.eq(true))
    .order(rank.desc())
    .load::<Page>(conn)
    .unwrap_or(vec![])
}

pub fn get_page_content_by_name(conn: &MysqlConnection, page_name: String) -> Option<Content> {
  pages
    .filter(name.eq(page_name))
    .first::<Page>(conn)
    .and_then(|page| {
      contents
        .filter(page_id.eq(page.id))
        .order(version.desc())
        .first::<Content>(&*conn)
    })
    .ok()
}