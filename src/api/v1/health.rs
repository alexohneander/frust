use actix_web::{get, web, Result, Responder};

#[get("/health")]
async fn check() -> Result<impl Responder> {    
    Ok(web::Json("OK"))
}
