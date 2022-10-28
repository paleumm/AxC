#![allow(unused_imports, unused_variables)]
use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use controller::*;
use tracing::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            // .allowed_origin("http://pile.internal/")
            // .allowed_origin_fn(|origin, _req_head| {
            //     origin.as_bytes().starts_with(b"http://pile.internal/")
            // })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(create)
            .service(delete)
            .service(exec)
            .service(create_user)
            .service(delete_user)
            .service(write)
            .service(test)
            .service(hello)
        // .service(run)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
