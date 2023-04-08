use actix_cors::Cors;
use actix_web::{delete, get, post, put, web::Json, App, HttpRequest, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{fs, io::Result};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub id: u16,
    pub title: String,
    pub page_amount: u16,
    pub cover_img_src: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BookId {
    pub id: u16,
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

pub fn save_books(books_cache: Vec<Book>) -> Result<()> {
    fs::write("./books.json", serde_json::to_string(&books_cache)?)?;
    Ok(())
}

pub fn get_books() -> Result<Vec<Book>> {
    let json_string = fs::read_to_string("./books.json")?;
    let book_list: Vec<Book> = serde_json::from_str(&json_string)?;
    Ok(book_list)
}

#[put("/books")]
async fn edit(req: HttpRequest, body: Json<Book>) -> impl Responder {
    let mut books_cache = get_books().unwrap();
    let index = books_cache
        .clone()
        .into_iter()
        .position(|b| b.id == body.id);
    let result = match index {
        Some(ind) => {
            books_cache.remove(ind);
            books_cache.push(Book::new(
                body.id,
                body.title.clone(),
                body.page_amount,
                body.cover_img_src.clone(),
            ));
            match save_books(books_cache) {
                Err(_) => String::from("Edit failed"),
                Ok(()) => String::from("Edit ok"),
            }
        }
        None => format!("Book with id {} not found", body.id),
    };
    println!(
        "PUT request received from {:?}, result : {}",
        req.peer_addr().unwrap(),
        result
    );
    result
}

//Make sure Content-Type is set to text/json in request header.
#[delete("/books")]
async fn delete(req: HttpRequest, body: Json<BookId>) -> impl Responder {
    let mut books_cache = get_books().unwrap();
    let index = books_cache
        .clone()
        .into_iter()
        .position(|b| b.id == body.id);
    let result = match index {
        Some(ind) => {
            books_cache.remove(ind);
            match save_books(books_cache) {
                Err(_) => String::from("Delete failed"),
                Ok(()) => String::from("Delete ok"),
            }
        }
        None => format!("Book with id {} not found", body.id),
    };
    println!(
        "DELETE request received from {:?}, result {}",
        req.peer_addr().unwrap(),
        result
    );
    result
}

//Make sure Content-Type is set to text/json in request header.
#[post("/books")]
async fn create(req: HttpRequest, body: Json<Book>) -> impl Responder {
    let mut books_cache = get_books().unwrap();
    books_cache.sort_by(|rhs, lhs| rhs.id.cmp(&lhs.id));
    let new_id = books_cache.last().unwrap().id + 1;
    books_cache.push(Book::new(
        new_id,
        body.title.clone(),
        body.page_amount,
        body.cover_img_src.clone(),
    ));
    let result = match save_books(books_cache) {
        Err(_) => String::from("Saving file failed"),
        Ok(_) => String::from("Creation ok"),
    };
    println!(
        "POST request received from {:?}, result {}",
        req.peer_addr().unwrap(),
        result
    );
    result
}

#[get("/books")]
async fn retrieve(req: HttpRequest) -> impl Responder {
    let result = match get_books() {
        Ok(book_list) => match serde_json::to_string(&book_list) {
            Ok(book_list_json_string) => book_list_json_string,
            Err(_) => String::from("Retrieval failed invalid file"),
        },
        Err(_) => String::from("Retrieval failed invalid file"),
    };
    println!(
        "GET request received from {:?}, result {}",
        req.peer_addr().unwrap(),
        result
    );
    result
}

#[actix_rt::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(retrieve)
            .service(create)
            .service(delete)
            .service(edit)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
