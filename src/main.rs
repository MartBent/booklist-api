use actix_web::{get, post, web::Json, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub id: u16,
    pub title: String,
    pub page_amount: u16,
    pub cover_img_src: String,
}

impl Book {
    pub fn new(id: u16, title: String, page_amount: u16, cover_img_src: String) -> Self {
        Self {
            id,
            title,
            page_amount,
            cover_img_src,
        }
    }
}

pub fn get_books() -> Result<Vec<Book>> {
    let json_string = fs::read_to_string("./books.json")?;
    let book_list: Vec<Book> = serde_json::from_str(&json_string)?;
    Ok(book_list)
}

//Make sure Content-Type is set to text/json in request header.
#[post("/books")]
async fn update_books(body: Json<Book>) -> impl Responder {
    let mut books_cache = get_books().unwrap();
    books_cache.sort_by(|rhs, lhs| rhs.id.cmp(&lhs.id));
    let new_id = books_cache.last().unwrap().id + 1;
    books_cache.push(Book::new(
        new_id,
        body.title.clone(),
        body.page_amount,
        body.cover_img_src.clone(),
    ));
    match fs::write("./books.json", serde_json::to_string(&books_cache).unwrap()) {
        Err(_) => String::from("Failed"),
        Ok(_) => String::from("Ok"),
    }
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
    HttpServer::new(|| App::new().service(books).service(update_books))
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
