use crate::routes;
use utoipa::OpenApi; // Import the missing `routes` module

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

openapi_paths_and_schemas!(
    routes::user::get_by_id,
    routes::user::add_user;
    routes::user::User,
    routes::user::CreateUser
);
