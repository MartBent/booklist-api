use actix_web::{get, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: u16,
    pub title: String,
    pub page_amount: u16,
}

impl Book {
    pub fn new(id: u16, title: String, page_amount: u16) -> Self {
        Self {
            id,
            title,
            page_amount,
        }
    }
}

pub fn get_books() -> Result<Vec<Book>> {
    let json_string = fs::read_to_string("./books.json")?;
    let mut book_list: Vec<Book> = serde_json::from_str(&json_string)?;
    book_list.push(Book { id: 2, title: String::from("The lord of the flies"), page_amount: 196 });
    Ok(book_list)
}

#[get("/books")]
async fn books() -> impl Responder {
    let res_body = match get_books() {
        Ok(book_list) => match serde_json::to_string(&book_list) {
            Ok(book_list_json_string) => book_list_json_string,
            Err(_) => String::from("Error decoding JSON file"),
        },
        Err(_) => String::from("{\"Error:\": \"Error fetching books from file\"}"),
    };
    res_body
}

#[actix_rt::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(books))
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
