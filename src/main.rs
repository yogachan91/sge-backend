use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use actix_web::http::header;
use dotenv::dotenv;

mod db;
mod models;
mod services;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::connect_db().await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://100.124.115.86:3000") // 🔥 frontend kamu
        //    .allowed_origin("http://localhost:3000") // 🔥 frontend kamu
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            .supports_credentials();

        App::new()
            .wrap(cors) // 🔥 WAJIB
            .app_data(web::Data::new(pool.clone()))
            .route("/upload", web::post().to(handlers::upload_excel))
            .route("/upload-part-number", web::post().to(handlers::upload_excel_part_number))
            .route("/export", web::get().to(handlers::download_excel))
            .route("/create-po-cs", web::post().to(handlers::upload_excel_create_po_cs))
            .route("/search-po", web::post().to(handlers::search_po_handler))
            .route("/upload-material", web::post().to(handlers::upload_excel_material))
            .route("/chat", web::post().to(handlers::chat_handler))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}