mod api;
mod service;
mod dao;
mod entity;

extern crate mongodb;
extern crate chrono;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer, http::header};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS", "PATCH", "DELETE", "PUT"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::ORIGIN])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .route("/article/list/{page}", web::get().to(api::article_api::find_article_list))
            .route("/article/create", web::post().to(api::article_api::create_article))
            .route("/article/update", web::post().to(api::article_api::update_article))
            .route("/article/delete", web::post().to(api::article_api::delete_article))
    }).bind("0.0.0.0:8000")?.run()
}