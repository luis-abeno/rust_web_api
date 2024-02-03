use actix_web::{web, HttpResponse, Responder};

pub async fn route_index() -> impl Responder {
    HttpResponse::Ok().body("Hello Index")
}

pub async fn route_a() -> impl Responder {
    HttpResponse::Ok().body("Hello Route A")
}

pub async fn route_b() -> impl Responder {
    HttpResponse::Ok().body("Hello Route B")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(route_index))
        .route("/a", web::get().to(route_a))
        .route("/b", web::get().to(route_b));
}
