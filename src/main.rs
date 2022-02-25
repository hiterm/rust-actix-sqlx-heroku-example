use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = fetch_port();
    let db_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

fn fetch_port() -> u16 {
    env::var("PORT")
        .map(|s| s.parse())
        .unwrap_or(Result::Ok(8080))
        .unwrap()
}

#[get("/")]
async fn hello(pool: web::Data<PgPool>) -> impl Responder {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().body(row.0.to_string())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
