use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;

use crate::services::process_excel;
use crate::services::process_excel_part_number;
use crate::services::process_excel_create_po_cs;
use crate::services::export_excel;
use crate::models::SearchPoRequest;
use crate::services::search_po;

pub async fn upload_excel(
    pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> HttpResponse {

    let mut file_bytes = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file_bytes.extend_from_slice(&data);
        }
    }

    match process_excel(&pool, file_bytes).await {
        Ok(_) => HttpResponse::Ok().body("Upload sukses"),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Error: {}", e)),
    }
}

pub async fn upload_excel_part_number(
    pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> HttpResponse {

    let mut file_bytes = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file_bytes.extend_from_slice(&data);
        }
    }

    match process_excel_part_number(&pool, file_bytes).await {
        Ok(_) => HttpResponse::Ok().body("Upload sukses"),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Error: {}", e)),
    }
}

pub async fn upload_excel_create_po_cs(
    pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> HttpResponse {

    let mut file_bytes = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file_bytes.extend_from_slice(&data);
        }
    }

    match process_excel_create_po_cs(&pool, file_bytes).await {
        Ok(_) => HttpResponse::Ok().body("Upload sukses"),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Error: {}", e)),
    }
}

pub async fn download_excel(pool: web::Data<PgPool>) -> HttpResponse {

    match export_excel(&pool).await {
        Ok(bytes) => {
            HttpResponse::Ok()
                .append_header((
                    "Content-Type",
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                ))
                .append_header((
                    "Content-Disposition",
                    "attachment; filename=\"data_master.xlsx\""
                ))
                .body(bytes)
        }
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(format!("Error: {}", e))
        }
    }
}

pub async fn search_po_handler(
    pool: web::Data<PgPool>,
    req: web::Json<SearchPoRequest>,
) -> HttpResponse {

    match search_po(&pool, req.filters.clone()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Error: {}", e)),
    }
}