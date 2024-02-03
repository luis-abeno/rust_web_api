mod routes;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(web::scope("/hello").configure(routes::hello::config))
                .service(web::scope("/user").configure(routes::user::config)),
        )
    })
    .bind(("127.0.0.1", 8080))?;

    println!("Server running on port 8080");

    server.run().await
}
