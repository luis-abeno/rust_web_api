// routes/mod.rs
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

pub async fn get_by_id(id: web::Path<u32>) -> impl Responder {
    let id = id.into_inner();

    let user = User {
        id,
        name: "Alice".to_string(),
        email: "alice@email.com".to_string(),
    };

    HttpResponse::Ok().json(user)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/{id}", web::get().to(get_by_id));
}
