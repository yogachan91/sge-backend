// use calamine::{open_workbook_auto, Reader, DataType};
// use sqlx::PgPool;
// use uuid::Uuid;
// use std::fs::File;
// use std::io::Write;
// use std::path::Path;
// use umya_spreadsheet::*;
// use std::io::Cursor;
// use chrono::{NaiveDate, Duration};
// use serde_json::json;

// use crate::models::{PoResponse, PoGroupRow};

// pub async fn process_excel(
//     pool: &PgPool,
//     file_bytes: Vec<u8>,
// ) -> Result<(), Box<dyn std::error::Error>> {

//     // 1. Simpan file sementara
//     let file_path = "temp.xlsx";
//     let mut file = File::create(file_path)?;
//     file.write_all(&file_bytes)?;

//     // 2. Baca Excel dari file
//     let mut workbook = open_workbook_auto(file_path)?;

//     let range = workbook
//         .worksheet_range_at(0)
//         .ok_or("Sheet not found")??;

//     for row in range.rows().skip(1) {

//         let nama = row.get(0).map(|c| c.to_string()).unwrap_or_default();
//         let kode = row.get(1).map(|c| c.to_string()).unwrap_or_default();
//         let contact = row.get(2).map(|c| c.to_string());
//         let no_hp = row.get(3).map(|c| c.to_string());
//         let alamat = row.get(4).map(|c| c.to_string());
//         let tipe = row.get(5).map(|c| c.to_string());

//         sqlx::query!(
//             r#"
//             INSERT INTO data_master (id, nama, kode, contact, no_hp, alamat, tipe)
//             VALUES ($1, $2, $3, $4, $5, $6, $7)
//             "#,
//             Uuid::new_v4(),
//             nama,
//             kode,
//             contact,
//             no_hp,
//             alamat,
//             tipe
//         )
//         .execute(pool)
//         .await?;
//     }

//     // 3. Hapus file sementara (optional tapi disarankan)
//     std::fs::remove_file(file_path)?;

//     Ok(())
// }

// pub async fn process_excel_part_number(
//     pool: &PgPool,
//     file_bytes: Vec<u8>,
// ) -> Result<(), Box<dyn std::error::Error>> {

//     // 1. Simpan file sementara
//     let file_path = "temp.xlsx";
//     let mut file = File::create(file_path)?;
//     file.write_all(&file_bytes)?;

//     // 2. Baca Excel dari file
//     let mut workbook = open_workbook_auto(file_path)?;

//     let range = workbook
//         .worksheet_range_at(0)
//         .ok_or("Sheet not found")??;

//     for row in range.rows().skip(1) {

//         let id_master = row.get(0).map(|c| c.to_string()).unwrap_or_default();
//         let tipe = row.get(1).map(|c| c.to_string());
//         let nomor = row.get(2).map(|c| c.to_string());

//         sqlx::query!(
//             r#"
//             INSERT INTO part_number (id, id_master, tipe, nomor)
//             VALUES ($1, $2, $3, $4)
//             "#,
//             Uuid::new_v4(),
//             id_master,
//             tipe,
//             nomor
//         )
//         .execute(pool)
//         .await?;
//     }

//     // 3. Hapus file sementara (optional tapi disarankan)
//     std::fs::remove_file(file_path)?;

//     Ok(())
// }

// fn excel_date_to_naive_date(value: &DataType) -> Option<NaiveDate> {
//     match value {
//         // format string: "2026-01-23"
//         DataType::String(s) => {
//             NaiveDate::parse_from_str(s, "%d-%m-%Y").ok()
//         }

//         // format excel serial number
//         DataType::Float(f) => {
//             let base_date = NaiveDate::from_ymd_opt(1899, 12, 30)?;
//             Some(base_date + Duration::days(*f as i64))
//         }

//         DataType::Int(i) => {
//             let base_date = NaiveDate::from_ymd_opt(1899, 12, 30)?;
//             Some(base_date + Duration::days(*i))
//         }

//         _ => None,
//     }
// }

// pub async fn process_excel_create_po_cs(
//     pool: &PgPool,
//     file_bytes: Vec<u8>,
// ) -> Result<(), Box<dyn std::error::Error>> {

//     // 1. Simpan file sementara
//     let file_path = "temp.xlsx";
//     let mut file = File::create(file_path)?;
//     file.write_all(&file_bytes)?;

//     // 2. Baca Excel dari file
//     let mut workbook = open_workbook_auto(file_path)?;

//     let range = workbook
//         .worksheet_range_at(0)
//         .ok_or("Sheet not found")??;

//     for row in range.rows().skip(1) {

//         let kode = row.get(0).map(|c| c.to_string()).unwrap_or_default();
//         let no_po = row.get(1).map(|c| c.to_string()).unwrap_or_default();
//         let part_number = row.get(2).map(|c| c.to_string()).unwrap_or_default();
//         let status = row.get(8).map(|c| c.to_string()).unwrap_or_default();

//         // ========================
//         // NUMBER (i64)
//         // ========================
//         let qty = match row.get(3) {
//             Some(DataType::Int(v)) => Some(*v as i64),
//             Some(DataType::Float(v)) => Some(*v as i64),
//             Some(DataType::String(s)) => s.parse::<i64>().ok(),
//             _ => None,
//         };

//         let qty_outstanding = match row.get(4) {
//             Some(DataType::Int(v)) => Some(*v as i64),
//             Some(DataType::Float(v)) => Some(*v as i64),
//             Some(DataType::String(s)) => s.parse::<i64>().ok(),
//             _ => None,
//         };

//         let harga_satuan = match row.get(5) {
//             Some(DataType::Int(v)) => Some(*v as i64),
//             Some(DataType::Float(v)) => Some(*v as i64),
//             Some(DataType::String(s)) => s.parse::<i64>().ok(),
//             _ => None,
//         };

//         let total = match row.get(6) {
//             Some(DataType::Int(v)) => Some(*v as i64),
//             Some(DataType::Float(v)) => Some(*v as i64),
//             Some(DataType::String(s)) => s.parse::<i64>().ok(),
//             _ => None,
//         };

//         // ========================
//         // DATE (NaiveDate)
//         // ========================
//         let tgl_po = row.get(7).and_then(excel_date_to_naive_date);
//         let delivery_time = row.get(9).and_then(excel_date_to_naive_date);
//         let target_prod = row.get(10).and_then(excel_date_to_naive_date);

//         sqlx::query!(
//             r#"
//             INSERT INTO po_cs (
//                 id, kode, no_po, part_number,
//                 qty, qty_outstanding, harga_satuan, total,
//                 tgl_po, status, delivery_time, target_prod
//             )
//             VALUES (
//                 $1,$2,$3,$4,
//                 $5,$6,$7,$8,
//                 $9,$10,$11,$12
//             )
//             "#,
//             Uuid::new_v4(),
//             kode,
//             no_po,
//             part_number,
//             qty,
//             qty_outstanding,
//             harga_satuan,
//             total,
//             tgl_po,
//             status,
//             delivery_time,
//             target_prod
//         )
//         .execute(pool)
//         .await?;
//     }

//     // 3. Hapus file sementara (optional tapi disarankan)
//     std::fs::remove_file(file_path)?;

//     Ok(())
// }

// pub async fn export_excel(pool: &PgPool) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

//     // 1. Ambil data dari DB
//     let rows = sqlx::query!(
//         r#"
//         SELECT id, nama, kode, contact, no_hp, alamat, tipe
//         FROM data_master
//         "#
//     )
//     .fetch_all(pool)
//     .await?;

//     // 2. Buat workbook
//     let mut book = new_file();
//     let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

//     // 3. Header
//     sheet.get_cell_mut("A1").set_value("id");
//     sheet.get_cell_mut("B1").set_value("nama");
//     sheet.get_cell_mut("C1").set_value("kode");
//     sheet.get_cell_mut("D1").set_value("contact");
//     sheet.get_cell_mut("E1").set_value("no_hp");
//     sheet.get_cell_mut("F1").set_value("alamat");
//     sheet.get_cell_mut("G1").set_value("tipe");

//     // 4. Isi data
//     for (i, row) in rows.iter().enumerate() {
//         let row_num = i + 2;

//         sheet.get_cell_mut(format!("A{}", row_num)).set_value(&row.id.to_string());
//         sheet.get_cell_mut(format!("B{}", row_num)).set_value(&row.nama);
//         sheet.get_cell_mut(format!("C{}", row_num)).set_value(&row.kode);
//         sheet.get_cell_mut(format!("D{}", row_num)).set_value(row.contact.as_deref().unwrap_or(""));
//         sheet.get_cell_mut(format!("E{}", row_num)).set_value(row.no_hp.as_deref().unwrap_or(""));
//         sheet.get_cell_mut(format!("F{}", row_num)).set_value(row.alamat.as_deref().unwrap_or(""));
//         sheet.get_cell_mut(format!("G{}", row_num)).set_value(row.tipe.as_deref().unwrap_or(""));
//     }

//     // 5. Convert ke bytes (memory, bukan file)
//     let mut buffer: Vec<u8> = Vec::new();
//     umya_spreadsheet::writer::xlsx::write_writer(&book, &mut Cursor::new(&mut buffer))?;

//     Ok(buffer)
// }

// pub async fn search_po(
//     pool: &PgPool,
//     filter: String,
// ) -> Result<Vec<PoResponse>, Box<dyn std::error::Error>> {

//     let rows: Vec<PoGroupRow> = if filter.trim().is_empty() {

//         // 🔥 latest 7 PO (group by)
//         sqlx::query_as!(
//             PoGroupRow,
//             r#"
//             SELECT 
//                 no_po,
//                 MAX(kode) as "kode?",
//                 MAX(part_number) as "part_number?",
//                 SUM(qty)::BIGINT as "qty?",
//                 SUM(total)::BIGINT as "total?",
//                 MAX(tgl_po) as "tgl_po?",
//                 MAX(delivery_time) as "delivery_time?"
//             FROM po_cs
//             GROUP BY no_po
//             ORDER BY MAX(tgl_po) DESC
//             LIMIT 7
//             "#
//         )
//         .fetch_all(pool)
//         .await?

//     } else {

//         // 🔍 search PO (grouped jadi 1)
//         sqlx::query_as!(
//             PoGroupRow,
//             r#"
//             SELECT 
//                 no_po,
//                 MAX(kode) as "kode?",
//                 MAX(part_number) as "part_number?",
//                 SUM(qty)::BIGINT as "qty?",
//                 SUM(total)::BIGINT as "total?",
//                 MAX(tgl_po) as "tgl_po?",
//                 MAX(delivery_time) as "delivery_time?"
//             FROM po_cs
//             WHERE no_po = $1
//             GROUP BY no_po
//             "#,
//             filter
//         )
//         .fetch_all(pool)
//         .await?
//     };

//     // ========================
//     // FORMAT DATE
//     // ========================
//     let format_date = |date: Option<NaiveDate>| -> String {
//         match date {
//             Some(d) => d.format("%d %b %Y").to_string(),
//             None => "-".to_string(),
//         }
//     };

//     // ========================
//     // MAPPING RESPONSE
//     // ========================
//     let mut results = Vec::new();

//     for row in rows {

//         let qty = row.qty.unwrap_or(0);

//         let item = PoResponse {
//             id: row.no_po,
//             client: "PT Sumitomo Wiring Systems".to_string(),
//             product: row.part_number.unwrap_or("-".to_string()),
//             qty,
//             deadline: format_date(row.delivery_time),
//             po_date: format_date(row.tgl_po),
//             current_stage: "materialCheck".to_string(),
//             stage_entered_date: format_date(row.tgl_po),

//             stages: json!({
//                 "materialCheck": {
//                     "status": "pending",
//                     "materials": [],
//                     "aiInsight": "Belum dilakukan pengecekan material."
//                 },
//                 "loa": { "status": "pending" },
//                 "production": {
//                     "status": "pending",
//                     "progress": 0,
//                     "target": qty
//                 },
//                 "delivery": {
//                     "status": "pending",
//                     "deliveryOrders": []
//                 },
//                 "closing": {
//                     "status": "pending",
//                     "invoiceAmount": row.total.unwrap_or(0),
//                     "paymentStatus": "unpaid",
//                     "poHealth": "good",
//                     "poHealthNote": "Masih aman",
//                     "aiInsight": "Perlu monitoring harga bahan baku."
//                 }
//             }),
//         };

//         results.push(item);
//     }

//     Ok(results)
// }

use calamine::{open_workbook_auto, Reader, DataType};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::fs::File;
use std::io::Write;
use std::io::Cursor;
use chrono::{NaiveDate, Duration};
use serde_json::json;
use umya_spreadsheet::*;

use crate::models::{PoResponse, PoGroupRow, PartNumberItem};

// ========================
// PROCESS EXCEL MASTER
// ========================
pub async fn process_excel(
    pool: &PgPool,
    file_bytes: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {

    let file_path = "temp.xlsx";
    let mut file = File::create(file_path)?;
    file.write_all(&file_bytes)?;

    let mut workbook = open_workbook_auto(file_path)?;
    let range = workbook.worksheet_range_at(0).ok_or("Sheet not found")??;

    for row in range.rows().skip(1) {

        let nama = row.get(0).map(|c| c.to_string()).unwrap_or_default();
        let kode = row.get(1).map(|c| c.to_string()).unwrap_or_default();
        let contact = row.get(2).map(|c| c.to_string());
        let no_hp = row.get(3).map(|c| c.to_string());
        let alamat = row.get(4).map(|c| c.to_string());
        let tipe = row.get(5).map(|c| c.to_string());

        sqlx::query(
            r#"
            INSERT INTO data_master (id, nama, kode, contact, no_hp, alamat, tipe)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(nama)
        .bind(kode)
        .bind(contact)
        .bind(no_hp)
        .bind(alamat)
        .bind(tipe)
        .execute(pool)
        .await?;
    }

    std::fs::remove_file(file_path)?;
    Ok(())
}

// ========================
// PROCESS EXCEL PART NUMBER
// ========================
pub async fn process_excel_part_number(
    pool: &PgPool,
    file_bytes: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {

    let file_path = "temp.xlsx";
    let mut file = File::create(file_path)?;
    file.write_all(&file_bytes)?;

    let mut workbook = open_workbook_auto(file_path)?;
    let range = workbook.worksheet_range_at(0).ok_or("Sheet not found")??;

    for row in range.rows().skip(1) {

        let id_master = row.get(0).map(|c| c.to_string()).unwrap_or_default();
        let tipe = row.get(1).map(|c| c.to_string());
        let nomor = row.get(2).map(|c| c.to_string());

        sqlx::query(
            r#"
            INSERT INTO part_number (id, id_master, tipe, nomor)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(id_master)
        .bind(tipe)
        .bind(nomor)
        .execute(pool)
        .await?;
    }

    std::fs::remove_file(file_path)?;
    Ok(())
}

// ========================
// HELPER DATE
// ========================
fn excel_date_to_naive_date(value: &DataType) -> Option<NaiveDate> {
    match value {
        DataType::DateTime(f) => {
            // ini format asli Excel date
            let base = NaiveDate::from_ymd_opt(1899, 12, 30)?;
            Some(base + Duration::days(*f as i64))
        }
        DataType::String(s) => {
            NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .or_else(|_| NaiveDate::parse_from_str(s, "%d-%m-%Y"))
                .ok()
        }
        DataType::Float(f) => {
            let base = NaiveDate::from_ymd_opt(1899, 12, 30)?;
            Some(base + Duration::days(*f as i64))
        }
        DataType::Int(i) => {
            let base = NaiveDate::from_ymd_opt(1899, 12, 30)?;
            Some(base + Duration::days(*i))
        }
        _ => None,
    }
}

// ========================
// HELPER PARSE NUMBER
// ========================
fn parse_i64(cell: Option<&DataType>) -> Option<i64> {
    match cell {
        Some(DataType::Int(v)) => Some(*v as i64),
        Some(DataType::Float(v)) => Some(*v as i64),
        Some(DataType::String(s)) => s.parse::<i64>().ok(),
        _ => None,
    }
}

// ========================
// PROCESS EXCEL PO
// ========================
pub async fn process_excel_create_po_cs(
    pool: &PgPool,
    file_bytes: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {

    let file_path = "temp.xlsx";
    let mut file = File::create(file_path)?;
    file.write_all(&file_bytes)?;

    let mut workbook = open_workbook_auto(file_path)?;
    let range = workbook.worksheet_range_at(0).ok_or("Sheet not found")??;

    for row in range.rows().skip(1) {

        let kode = row.get(0).map(|c| c.to_string()).unwrap_or_default();
        let no_po = row.get(1).map(|c| c.to_string()).unwrap_or_default();
        let part_number = row.get(2).map(|c| c.to_string()).unwrap_or_default();
        let status = row.get(8).map(|c| c.to_string()).unwrap_or_default();

        let qty = parse_i64(row.get(3));
        let qty_outstanding = parse_i64(row.get(4));
        let harga_satuan = parse_i64(row.get(5));
        let total = parse_i64(row.get(6));

        let tgl_po = row.get(7).and_then(excel_date_to_naive_date);
        let delivery_time = row.get(9).and_then(excel_date_to_naive_date);
        let target_prod = row.get(10).and_then(excel_date_to_naive_date);

        let no_spk = row.get(11).map(|c| c.to_string()).unwrap_or_default();
        let qty_terdeliver = parse_i64(row.get(12));
        let tanggal_delivery = row.get(13).and_then(excel_date_to_naive_date);
        let status_spk = row.get(14).map(|c| c.to_string()).unwrap_or_default();
        let status_delivery = row.get(15).map(|c| c.to_string()).unwrap_or_default();
        let status_material = row.get(16).map(|c| c.to_string()).unwrap_or_default();

        sqlx::query(
            r#"
            INSERT INTO po_cs (
                id, kode, no_po, part_number,
                qty, qty_outstanding, harga_satuan, total,
                tgl_po, status, delivery_time, target_prod,
                no_spk, qty_terdeliver, tanggal_delivery, status_delivery, status_spk, status_material
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(kode)
        .bind(no_po)
        .bind(part_number)
        .bind(qty)
        .bind(qty_outstanding)
        .bind(harga_satuan)
        .bind(total)
        .bind(tgl_po)
        .bind(status)
        .bind(delivery_time)
        .bind(target_prod)
        .bind(no_spk)
        .bind(qty_terdeliver)
        .bind(tanggal_delivery)
        .bind(status_delivery)
        .bind(status_spk)
        .bind(status_material)
        .execute(pool)
        .await?;
    }

    std::fs::remove_file(file_path)?;
    Ok(())
}

// ========================
// EXPORT EXCEL (FIX TOTAL)
// ========================
pub async fn export_excel(
    pool: &PgPool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let rows = sqlx::query(
        "SELECT id, nama, kode, contact, no_hp, alamat, tipe FROM data_master"
    )
    .fetch_all(pool)
    .await?;

    let mut book = new_file();
    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

    let headers = ["id","nama","kode","contact","no_hp","alamat","tipe"];

    // ✅ FIX HEADER
    for (i, h) in headers.iter().enumerate() {
        sheet
            .get_cell_mut(((i + 1) as u32, 1u32))
            .set_value(*h);
    }

    // ✅ FIX DATA
    for (i, row) in rows.iter().enumerate() {
        let r = (i + 2) as u32;

        sheet.get_cell_mut((1u32, r)).set_value(row.get::<Uuid,_>("id").to_string());
        sheet.get_cell_mut((2u32, r)).set_value(row.get::<String,_>("nama"));
        sheet.get_cell_mut((3u32, r)).set_value(row.get::<String,_>("kode"));
        sheet.get_cell_mut((4u32, r)).set_value(row.get::<Option<String>,_>("contact").unwrap_or_default());
        sheet.get_cell_mut((5u32, r)).set_value(row.get::<Option<String>,_>("no_hp").unwrap_or_default());
        sheet.get_cell_mut((6u32, r)).set_value(row.get::<Option<String>,_>("alamat").unwrap_or_default());
        sheet.get_cell_mut((7u32, r)).set_value(row.get::<Option<String>,_>("tipe").unwrap_or_default());
    }

    let mut buffer = Vec::new();
    umya_spreadsheet::writer::xlsx::write_writer(&book, &mut Cursor::new(&mut buffer))?;

    Ok(buffer)
}

pub async fn process_excel_material(
    pool: &PgPool,
    file_bytes: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {

    let file_path = "temp_material.xlsx";
    let mut file = File::create(file_path)?;
    file.write_all(&file_bytes)?;

    let mut workbook = open_workbook_auto(file_path)?;
    let range = workbook.worksheet_range_at(0).ok_or("Sheet not found")??;

    for row in range.rows().skip(1) {

        let kode = row.get(0).map(|c| c.to_string()).unwrap_or_default();
        let no_po = row.get(1).map(|c| c.to_string()).unwrap_or_default();
        let part_number = row.get(2).map(|c| c.to_string()).unwrap_or_default();
        let nm_material = row.get(3).map(|c| c.to_string());
        let tipe = row.get(4).map(|c| c.to_string());
        let unit = row.get(5).map(|c| c.to_string());

        let butuh_qty = parse_i64(row.get(7));
        let stock_gudang_qty = parse_i64(row.get(8));
        let sisa_stock_gudang = parse_i64(row.get(9));
        let allocated = parse_i64(row.get(10));

        let status = row.get(11).map(|c| c.to_string());

        let satuan = row.get(6).and_then(|c| match c {
            DataType::Float(v) => Some(*v),
            DataType::Int(v) => Some(*v as f64),
            DataType::String(s) => s.parse::<f64>().ok(),
            _ => None,
        });

        sqlx::query(
            r#"
            INSERT INTO material (
                id, kode, no_po, part_number,
                nm_material, tipe, unit,
                butuh_qty, stock_gudang_qty, sisa_stock_gudang, allocated,
                status, satuan
            )
            VALUES (
                $1,$2,$3,$4,
                $5,$6,$7,
                $8,$9,$10,$11,
                $12,$13
            )
            "#
        )
        .bind(Uuid::new_v4())
        .bind(kode)
        .bind(no_po)
        .bind(part_number)
        .bind(nm_material)
        .bind(tipe)
        .bind(unit)
        .bind(butuh_qty)
        .bind(stock_gudang_qty)
        .bind(sisa_stock_gudang)
        .bind(allocated)
        .bind(status)
        .bind(satuan)
        .execute(pool)
        .await?;
    }

    std::fs::remove_file(file_path)?;
    Ok(())
}

// ========================
// SEARCH PO
// ========================
pub async fn search_po(
    pool: &PgPool,
    filter: String,
) -> Result<Vec<PoResponse>, Box<dyn std::error::Error>> {

    let rows: Vec<PoGroupRow> = sqlx::query_as::<_, PoGroupRow>(
        r#"
        SELECT 
        a.no_po,
        b.nama as vendor,
        SUM(a.qty)::BIGINT as qty,
        SUM(a.total)::BIGINT as total,
        MAX(a.tgl_po) as tgl_po,
        MAX(a.delivery_time) as delivery_time,

        json_agg(
            json_build_object(
                'nama', a.part_number,
                'qty', a.qty,
                'tgl_po', to_char(a.tgl_po, 'DD Mon YYYY'),
                'delivery_time', to_char(a.delivery_time, 'DD Mon YYYY'),
                'qty_terdeliver', a.qty_terdeliver,
                'tanggal_delivery', to_char(a.tanggal_delivery, 'DD Mon YYYY'),
                'status', a.status
            )
        ) as part_numbers,

        -- ✅ NEW MATERIALS
        (
            SELECT json_agg(
                json_build_object(
                    'name', m.nm_material,
                    'required', m.butuh_qty,
                    'unit', m.unit,
                    'status', m.status,
                    'currentStock', m.stock_gudang_qty,
                    'allocated', m.allocated
                )
            )
            FROM material m
            WHERE m.no_po = a.no_po
        ) as materials

    FROM po_cs a
    JOIN data_master b ON a.kode = b.kode

    WHERE ($1 = '' OR a.no_po = $1 OR b.nama = $1)

    GROUP BY a.no_po, b.nama
    ORDER BY MAX(a.tgl_po) DESC
    LIMIT 7
        "#
    )
    .bind(filter.clone())
    .fetch_all(pool)
    .await?;

    let format_date = |d: Option<NaiveDate>| {
        d.map(|x| x.format("%d %b %Y").to_string())
            .unwrap_or("-".to_string())
    };

    let mut results = Vec::new();

    for row in rows {

        let qty = row.qty.unwrap_or(0);
        let total = row.total.unwrap_or(0);

        // 🔥 parsing JSON → Vec struct
        let part_numbers: Vec<PartNumberItem> =
            serde_json::from_value(row.part_numbers).unwrap_or(vec![]);

        let materials: Vec<serde_json::Value> =
            row.materials
            .map(|m| serde_json::from_value(m).unwrap_or(vec![]))
            .unwrap_or(vec![]);

        let product = part_numbers
            .get(0)
            .map(|p| p.nama.clone())
            .unwrap_or("-".to_string());

        let delivery_orders: Vec<serde_json::Value> = part_numbers
    .iter()
    .map(|p| {
        json!({
            "doNumber": p.nama,
            "qty": p.qty.unwrap_or(0),
            "status": "pending",
            "scheduledDate": p.delivery_time,
            "courier": "-"
        })
    })
    .collect();

        results.push(PoResponse {
            id: row.no_po,
            client: row.vendor,
            product,
            qty,
            deadline: format_date(row.delivery_time),
            po_date: format_date(row.tgl_po),
            current_stage: "materialCheck".to_string(),
            stage_entered_date: format_date(row.tgl_po),

            part_number: part_numbers, // ✅ ARRAY

            stages: json!({
                "materialCheck": {
                    "status": "pending",
                    "materials": materials,
                    "aiInsight": "Loading..."
                },
                "loa": {
                    "status": "pending",
                    "loaNumber": "LoA-SGE-2026-002",
                    "issuedDate": "-",
                    "referencedMaterials": "-",
                    "assignedJobTask": "-",
                },
                "production": {
                    "status": "pending",
                    "progress": 0,
                    "target": qty
                },
                "delivery": {
                    "status": "pending",
                    "deliveryOrders": delivery_orders
                },
                "closing": {
                    "status": "pending",
                    "invoiceAmount": total,
                    "paymentStatus": "unpaid",
                    "poHealth": "good",
                    "poHealthNote": "Masih aman",
                    "aiInsight": "Perlu monitoring harga bahan baku."
                }
            }),
        });
    }

    Ok(results)
}