use crate::controller::{delete_link, list_links, redirect, shorten};
use actix_web::{App, HttpServer, web};

mod controller;
mod db;
mod model;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let pg_pool = db::init_db().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_pool.clone()))
            .service(list_links)
            .service(redirect)
            .service(shorten)
            .service(delete_link)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
