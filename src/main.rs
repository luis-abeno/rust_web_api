mod api_doc;
mod routes;

use actix_web::{
    web::{self},
    App, HttpResponse, HttpServer,
};
use api_doc::ApiDoc;
use std::net::Ipv4Addr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            // A route just to guide user and dont leave the page empty
            .route("/", web::get().to(welcome))
            .route("/health", web::get().to(health_check)) // Health check
            // Main routes
            .service(
                web::scope("/api")
                    .service(web::scope("/hello").configure(routes::hello::config))
                    .service(web::scope("/user").configure(routes::user::config)),
            )
            // OpenAPI swagger docs
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?;

    println!("Server running on port 8080");

    server.run().await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Healthy!")
}

async fn welcome() -> HttpResponse {
    // Render a html with a welcome message and a link to the swagger ui
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <html>
                <head>
                    <title>Welcome to the Actix web API</title>
                </head>
                <body>
                    <h1>Welcome to the Actix web API</h1>
                    <p>Click <a href="/swagger-ui/index.html">here</a> to view the API documentation</p>
                    <p>The health check route is "/health"</p>
                </body>
            </html>
        "#,
        )
}
