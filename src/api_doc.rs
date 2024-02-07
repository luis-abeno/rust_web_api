use crate::routes;
use utoipa::OpenApi;

macro_rules! openapi_paths_and_schemas {
    ($($path:expr),*; $($schema:expr),*) => {
        #[derive(OpenApi)]
        #[openapi(
            info(description = "Rust web api"),
            paths($($path),*),
            components(schemas($($schema),*))
        )]
        pub struct ApiDoc;
    };
}

// TODO: Check if there's a way to improve this macro or maybe it's ok.
// It's weird to define all routes here, but it's the only way I found to make it work while refactoring.
// Maybe in a microservice architecture it's works well, if you're using a monolithic architecture
// it's better to define the routes in the main.rs
openapi_paths_and_schemas!(
    routes::user::get_by_id,
    routes::user::add_user,
    routes::user::update_user,
    routes::user::delete_user;
    routes::user::User,
    routes::user::UserMutable
);
