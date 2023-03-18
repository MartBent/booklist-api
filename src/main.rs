use actix_web::{get, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::io::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    pub title: String,
    pub page_amount: u16,
}

pub fn get_books() -> Vec<Book> {
    let mut book_list: Vec<Book> = Vec::new();
    book_list.push(Book::new(String::from("Misery"), 261));
    book_list.push(Book::new(String::from("Misery"), 261));
    book_list.push(Book::new(String::from("Lord of the flies"), 210));
    book_list.push(Book::new(
        String::from("Harry Potter: The chamber of secrets"),
        513,
    ));
    book_list.push(Book::new(String::from("Vals alarm"), 400));
    book_list
}

impl Book {
    pub fn new(title: String, page_amount: u16) -> Self {
        Self {
            title: title,
            page_amount: page_amount,
        }
    }
}

#[get("/books")]
async fn books() -> impl Responder {
    let res_body = serde_json::to_string(&get_books()).unwrap();
    res_body
}

#[actix_rt::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(books))
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
