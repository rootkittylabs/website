use actix_files::NamedFile;
use actix_web::Result;
use std::env;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    let port = env::var("PORT").unwrap_or("8080".to_string());

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}