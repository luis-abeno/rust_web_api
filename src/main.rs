mod api_doc;
mod routes;

use actix_web::{web, App, HttpServer};
use api_doc::ApiDoc;
use std::net::Ipv4Addr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
