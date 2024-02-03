use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct User {
    id: u32,
    name: String,
    email: String,
}

/// Get user by id
/// Get user from database by user id
#[utoipa::path(
        get,
        path = "/api/user/{id}",
        responses(
            (status = 200, description = "User found successfully", body = User),
            (status = NOT_FOUND, description = "User was not found")
        ),
        params(
            ("id" = u64, Path, description = "User database id to get user for"),
        )
    )]
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
