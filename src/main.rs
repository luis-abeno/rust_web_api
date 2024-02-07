mod api_doc;
mod models;
mod routes;

use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use api_doc::ApiDoc;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::net::Ipv4Addr;
use std::{error::Error, sync::Mutex};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn connect_to_mongodb() -> Result<Client, Box<dyn Error>> {
    let client_uri = "mongodb://localhost:27017";
    let client_options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the MongoDB database
    let mongo_db_client: Client = connect_to_mongodb().await.unwrap();
    let mongo_db_client: Data<Mutex<Client>> = Data::new(Mutex::new(mongo_db_client));

    // TODO: Connect to the MySQL database
    // TODO: Connect to the PostgreSQL database

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&mongo_db_client)) // Add the mongodb client to the app data to make it available to the routes
            .route("/", web::get().to(welcome)) // A route just to guide user and dont leave the page empty
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

    server.run().await?;
    Ok(())
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
