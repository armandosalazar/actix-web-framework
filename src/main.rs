use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use actix_web::http::header::ContentType;
use actix_web::web::get;
use mysql::{Pool, PooledConn};
use mysql::prelude::Queryable;
use serde::Serialize;

fn get_user_by_id(id: u8) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut conn = Pool::new("mysql://root:root@localhost:8889/test_db")?.get_conn()?;
    let users: Vec<(String, String, String, String)> = conn.query("SELECT * FROM user WHERE id = 1")?;
    Ok(())
}


#[derive(Serialize)]
struct User {
    id: u8,
    username: String,
    password: String,
    created: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .body(req_body)
}

#[get("/json")]
async fn json() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json()).json(User {
        id: 1,
        username: String::from("armando@user"),
        password: String::from("12345"),
        created: "".to_string(),
    })
}

#[get("/user/{id}")]
async fn get_user(id: web::Path<u8>) -> impl Responder {
    let r = get_user_by_id(1);
    format!("Hello {:?}!", id)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(json)
            .service(get_user)
            .route("/hey", web::get().to(manual_hello))
    }).bind(("localhost", 8080))?.run().await
}
