use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Create user struct to be used in the response body
/// It represents all user data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

/// Create user struct to be used in the request body
/// This struct will be used to deserialize the request body
/// can be used to create or update a user for example
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserMutable {
    pub name: String,
    pub email: String,
}
