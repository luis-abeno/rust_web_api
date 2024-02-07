use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

/// Create user struct to be used in the request body
/// This struct will be used to deserialize the request body
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserMutable {
    pub name: String,
    pub email: String,
}
