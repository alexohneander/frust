use actix_web::{get, web, Result, Responder, post};
use diesel::prelude::*;
use serde::Deserialize;

use self::schema::feeds::dsl::*;

use crate::models::context::establish_connection;
use crate::models::feed::{Feed, create_feed};
use crate::schema;

#[derive(Deserialize)]
struct AddRepository {
    title: String,
    url: String,
}

/// Get Feeds
///
/// Get Feeds from database
#[utoipa::path(
    get,
    path = "/api/v1/feeds",
    responses(
        (status = 200, description = "Feeds found succesfully", body = Vec<Feed>),
        (status = NOT_FOUND, description = "Feeds was not found")
    )
)]
#[get("/api/v1/feeds")]
async fn get() -> Result<impl Responder> {    
    let connection = &mut establish_connection();
    let results = feeds
        .limit(5)
        .select(Feed::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} feeds", results.len());

    //return results as json
    Ok(web::Json(results))
}

/// Add Feed
///
/// Add Feed to database
#[utoipa::path(
    post,
    path = "/api/v1/feeds",
    responses(
        (status = 200, description = "Feed added succesfully", body = Feed),
        (status = NOT_FOUND, description = "Feed was not added")
    ),
    request_body = NewFeed
)]
#[post("/api/v1/feeds")]
async fn post(req_body: web::Json<AddRepository>) -> Result<impl Responder> {
    // Get request body
    if req_body.title.is_empty() || req_body.url.is_empty() {
        println!("Title or URL is empty");
    }

    let connection = &mut establish_connection();
    let result = create_feed(connection, &req_body.title, &req_body.url);

    Ok(web::Json(result))
}