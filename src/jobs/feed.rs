use diesel::prelude::*;
use self::schema::feeds::dsl::*;

use crate::models::context::establish_connection;
use crate::models::feed::Feed;
use crate::schema;
use crate::services::feed::FeedService;


pub fn check_feed_types(name: &str) {
    let connection = &mut establish_connection();
    let results = feeds
        .select(Feed::as_select())
        .filter(feedtype.eq("Unknown"))
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} feeds", &results.len());

    for feed in results {
        FeedService::check_feed_type(feed);
    }
}