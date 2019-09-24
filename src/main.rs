mod api;
mod service;
mod dao;
mod entity;

extern crate mongodb;
extern crate chrono;

use actix_web::{middleware, web, App, HttpServer};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/topic/list", web::get().to(api::topic_api::find_topic_list))
            .route("/topic/create", web::post().to(api::topic_api::create_topic))
            .route("/topic/update", web::post().to(api::topic_api::update_topic))
            .route("/topic/delete", web::post().to(api::topic_api::delete_topic))
            .route("/article/create", web::post().to(api::article_api::create_article))
    }).bind("127.0.0.1:8080")?.run()
}