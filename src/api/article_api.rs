use crate::api::request::article_create_request::ArticleCreateRequest;
use crate::service::article_service;

use actix_web::{web, Result};
use mongodb::Bson;

pub fn find_article_list() {

}

pub fn create_article(info: web::Json<ArticleCreateRequest>) -> Result<web::Json<Bson>> {
    let bson = article_service::create_article(&info.title, &info.topic_id, &info.tags, &info.markdown);
    Ok(web::Json(bson))
}

pub fn update_article() {

}

pub fn delete_article() {

}

