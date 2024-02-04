use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    id: u32,
    name: String,
    email: String,
}

/// Create user struct to be used in the request body
/// This struct will be used to deserialize the request body
/// into a Rust struct
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    name: String,
    email: String,
}

/// Get user by id
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

/// Add new user
#[utoipa::path(
        post,
        path = "/api/user/add",
        responses(
            (status = 201, description = "User added successfully", body = User),
            (status = BAD_REQUEST, description = "Invalid user data")
        ),
    )]
pub async fn add_user(user_data: web::Json<CreateUser>) -> impl Responder {
    let user = User {
        id: 1,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };

    HttpResponse::Created().json(user)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/{id}", web::get().to(get_by_id))
        .route("/add", web::post().to(add_user));
}
