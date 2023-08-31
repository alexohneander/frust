use diesel::prelude::*;
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

use crate::schema::feeds;

#[derive(Queryable, Selectable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::feeds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Feed {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub feedtype: String,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = feeds)]
pub struct NewFeed<'a> {
    pub title: &'a str,
    pub url: &'a str,
}

pub fn create_feed(conn: &mut PgConnection, title: &str, url: &str) -> Feed {
    let new_feed = NewFeed { title, url };

    diesel::insert_into(feeds::table)
        .values(&new_feed)
        .returning(Feed::as_returning())
        .get_result(conn)
        .expect("Error saving new Feed")
}