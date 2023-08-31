use diesel::prelude::*;
use self::schema::feeds::dsl::*;

use crate::models::context::establish_connection;
use crate::models::feed::Feed;
use crate::schema;

pub struct FeedService {}

impl FeedService {
    pub fn check_feed_type(feed: Feed) {
        // Request feed.url and check if it is a valid RSS or Atom feed
        // If it is, update the feedtype in the database
        // If it is not, update the feedtype in the database to "Invalid"
        println!("Checking feed type for {}", feed.url);
        let resp = reqwest::blocking::get(feed.url).unwrap();
        let connection = &mut establish_connection();
        
        if resp.status().is_success() {
            let res = diesel::update(feeds.find(feed.id))
                .set(feedtype.eq("RSS"))
                .returning(Feed::as_returning())
                .get_result(connection)
                .unwrap();
            println!("Feed type updated to {}", res.feedtype);
        } else {
            println!("Feed is invalid");
        }
    }
}