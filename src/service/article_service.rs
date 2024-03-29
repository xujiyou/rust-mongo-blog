use crate::dao::article_repository;
use crate::entity::article::Article;

use mongodb::Bson;
use mongodb::ordered::OrderedDocument;
use mongodb::coll::results::{UpdateResult, DeleteResult};
use crate::api::request::article_find_request::ArticleFindRequest;

pub fn find_article_list(page: u32, request: ArticleFindRequest) -> Vec<OrderedDocument> {
    let cursor = article_repository::find_article_list(page, request);
    let mut doc_list: Vec<OrderedDocument> = vec![];
    for result in cursor {
        if let Ok(item) = result {
            doc_list.push(item);
        }
    }
    doc_list
}

pub fn create_article(article: Article) -> Bson {
    let result = article_repository::create_article(article);
    result.inserted_id.unwrap()
}

pub fn update_article(id: &String, title: &String, category: &String, technology: &String, tags: &Vec<String>, introduce: &String) -> UpdateResult {
    article_repository::update_article(id, title, category, technology, tags, introduce)
}

pub fn delete_article(id: &String) -> DeleteResult {
    article_repository::delete_article(id)
}