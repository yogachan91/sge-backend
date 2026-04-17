use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMaster {
    pub id: Uuid,
    pub nama: String,
    pub kode: String,
    pub contact: Option<String>,
    pub no_hp: Option<String>,
    pub alamat: Option<String>,
    pub tipe: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartNumber {
    pub id: Uuid,
    pub id_master: String,
    pub tipe: Option<String>,
    pub nomor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoCs {
    pub id: Uuid,
    pub kode: String,
    pub no_po: String,
    pub part_number: String,
    pub qty: Option<i64>,
    pub qty_outstanding: Option<i64>,
    pub harga_satuan: Option<i64>,
    pub total: Option<i64>,
    pub tgl_po: Option<NaiveDate>,
    pub status: String,
    pub delivery_time: Option<NaiveDate>,
    pub target_prod: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct SearchPoRequest {
    pub filters: String,
}

#[derive(Debug, Serialize)]
pub struct PoResponse {
    pub id: String,
    pub client: String,
    pub product: String,
    pub qty: i64,
    pub deadline: String,
    pub po_date: String,
    pub current_stage: String,
    pub stage_entered_date: String,
    pub stages: serde_json::Value,
}

#[derive(Debug)]
pub struct PoRow {
    pub id: Uuid,
    pub kode: String,
    pub no_po: String,
    pub part_number: String,
    pub qty: Option<i64>,
    pub qty_outstanding: Option<i64>,
    pub harga_satuan: Option<i64>,
    pub total: Option<i64>,
    pub tgl_po: Option<NaiveDate>,
    pub status: String,
    pub delivery_time: Option<NaiveDate>,
    pub target_prod: Option<NaiveDate>,
}

#[derive(Debug, FromRow)]
pub struct PoGroupRow {
    pub no_po: String,
    pub kode: Option<String>,
    pub part_number: Option<String>,
    pub qty: Option<i64>,
    pub total: Option<i64>,
    pub tgl_po: Option<NaiveDate>,
    pub delivery_time: Option<NaiveDate>,
}