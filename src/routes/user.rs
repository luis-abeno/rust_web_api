use crate::models::user::{User, UserMutable};
use actix_web::{web, HttpResponse, Responder};

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
pub async fn add_user(user_data: web::Json<UserMutable>) -> impl Responder {
    let user = User {
        id: 1,
        name: user_data.0.name.clone(),
        email: user_data.0.email.clone(),
    };

    HttpResponse::Created().json(user)
}

/// Update a user
#[utoipa::path(
        put,
        path = "/api/user/{id}",
        responses(
            (status = 200, description = "User updated successfully", body = User),
            (status = BAD_REQUEST, description = "Invalid user data"),
            (status = NOT_FOUND, description = "User was not found")
        ),
        params(
            ("id" = u64, Path, description = "User database id to update user for"),
        )
    )]
pub async fn update_user(id: web::Path<u32>, user_data: web::Json<UserMutable>) -> impl Responder {
    let id = id.into_inner();

    let user = User {
        id,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };

    HttpResponse::Ok().json(user)
}

/// Delete user
#[utoipa::path(
        delete,
        path = "/api/user/{id}",
        responses(
            (status = 204, description = "User deleted successfully"),
            (status = NOT_FOUND, description = "User was not found")
        ),
        params(
            ("id" = u64, Path, description = "User database id to delete user for"),
        )
    )]
pub async fn delete_user(id: web::Path<u32>) -> impl Responder {
    let id: u32 = id.into_inner();
    println!("Deleting user with id: {}", id);
    HttpResponse::NoContent().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/{id}", web::get().to(get_by_id))
        .route("/add", web::post().to(add_user))
        .route("/{id}", web::put().to(update_user))
        .route("/{id}", web::delete().to(delete_user));
}
