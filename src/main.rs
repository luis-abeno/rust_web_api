mod routes;

use std::net::Ipv4Addr;

use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(description = "Rust web api"),
    paths(routes::user::get_by_id, routes::user::add_user),
    components(schemas(routes::user::User, routes::user::CreateUser))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(web::scope("/hello").configure(routes::hello::config))
                    .service(web::scope("/user").configure(routes::user::config)),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?;

    println!("Server running on port 8080");

    server.run().await
}
