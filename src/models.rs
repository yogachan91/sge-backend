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
    pub no_spk: String,
    pub qty_terdeliver: Option<i64>,
    pub tanggal_delivery: Option<NaiveDate>,
    pub status_delivery: String,
    pub status_spk: String,
    pub status_material: String,
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
    pub part_number: Vec<PartNumberItem>,
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
    pub no_spk: String,
    pub qty_terdeliver: Option<i64>,
    pub tanggal_delivery: Option<NaiveDate>,
    pub status_delivery: String,
    pub status_spk: String,
    pub status_material: String,
}

#[derive(Debug, FromRow)]
pub struct PoGroupRow {
    pub no_po: String,
    pub vendor: String,
    pub part_numbers: serde_json::Value,
    pub materials: Option<serde_json::Value>,
    pub qty: Option<i64>,
    pub total: Option<i64>,
    pub tgl_po: Option<NaiveDate>,
    pub delivery_time: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartNumberItem {
    pub nama: String,
    pub qty: Option<i64>,
    pub tgl_po: String,
    pub delivery_time: String,
    pub qty_terdeliver: Option<i64>,
    pub tanggal_delivery: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}