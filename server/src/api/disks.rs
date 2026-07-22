use core::sync;

use actix_web::{HttpResponse, web::{self, Json}};
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
    let disk_type = req.disk_type.as_deref().unwrap_or("fixed");

    match crate::db::disks::insert(&conn, &req.label, &req.mount_path, disk_type) {
        Ok(disk_id) => {
            // Для removable дисков создаём маркер в корне
            if disk_type == "removable" {
                if let Err(e) = crate::storage::disks::write_marker(&req.mount_path, &disk_id) {
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": format!("Диск зарегистрирован, но не удалось создать маркер: {}", e)
                    }));
                }
            }

            HttpResponse::Created().json(serde_json::json!({
                "disk_id": disk_id,
                "message": "Диск зарегистрирован"
            }))
        }
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

pub async fn check_disks(
    conn: web::Data<std::sync::Mutex<Connection>>,
) -> HttpResponse {
    let conn = conn.lock().unwrap();

    match crate::storage::disks::sync_disks(&conn) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "message": "проверка дисков выполнена"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

#[derive(Deserialize)]
pub struct AddMediaRootRequest {
    pub relative_path: String,
}

pub async fn add_media_root(
    conn: web::Data<std::sync::Mutex<Connection>>,
    path: web::Path<String>,
    req: web::Json<AddMediaRootRequest>,
) -> HttpResponse {
    let disk_id = path.into_inner();
    let conn = conn.lock().unwrap();

    // Проверяем, что диск существует
    match crate::db::disks::find_by_id(&conn, &disk_id) {
        Ok(Some(_)) => {}
        Ok(None) => return HttpResponse::NotFound().json(serde_json::json!({
            "error": "Диск не найден"
        })),
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }

    match crate::db::media_roots::insert(&conn, &disk_id, &req.relative_path) {
        Ok(id) => HttpResponse::Created().json(serde_json::json!({
            "id": id,
            "message": "Медиа-папка добавлена"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}