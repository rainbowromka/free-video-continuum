use actix_web::{web, HttpResponse};
use rusqlite::Connection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddDiskRequest {
    pub label: String,
    pub mount_path: String,
    pub disk_type: Option<String>,
}

pub async fn add_disk(
    conn: web::Data<std::sync::Mutex<Connection>>,
    req: web::Json<AddDiskRequest>,
) -> HttpResponse {
    let conn = conn.lock().unwrap();
    let disk_type = req.disk_type.as_deref().unwrap_or("physical");

    match crate::db::disks::insert(&conn, &req.label, &req.mount_path, disk_type) {
        Ok(disk_id) => HttpResponse::Created().json(serde_json::json!({
            "disk_id": disk_id,
            "message": "Диск зарегистрирован"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

pub async fn list_disks(
    conn: web::Data<std::sync::Mutex<Connection>>,
) -> HttpResponse {
    let conn = conn.lock().unwrap();

    match crate::db::disks::list_all(&conn) {
        Ok(disks) => {
            let result: Vec<serde_json::Value> = disks
                .iter()
                .map(|d| serde_json::json!({
                    "disk_id": d.disk_id,
                    "label": d.label,
                    "mount_path": d.mount_path,
                    "disk_type": d.disk_type,
                    "is_available": d.is_available,
                }))
                .collect();

            HttpResponse::Ok().json(result)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}