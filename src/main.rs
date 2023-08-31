use actix_web::{App, HttpServer, middleware::{self, Logger}};
use cronjob::CronJob;
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::v1::feed;
use crate::handlers::hello_world;
use crate::api::v1::health;
use crate::models::feed::Feed;
use crate::models::feed::NewFeed;
use crate::jobs::feed::check_feed_types;

mod models;
mod api;
mod schema;
mod handlers;
mod jobs;
mod services;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    #[derive(OpenApi)]
    #[openapi(paths(feed::get, feed::post), components(schemas(Feed, NewFeed)))]
    struct ApiDoc;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Init CronJobs
    let mut check_feed_cron = CronJob::new("Test Cron", check_feed_types);
    check_feed_cron.seconds("10");
    CronJob::start_job_threaded(check_feed_cron);

    HttpServer::new(|| {
        App::new()
            // Middlewares
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            // Routes
            .service(hello_world::hello)
            .service(health::check)
            .service(feed::get)
            .service(feed::post)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}