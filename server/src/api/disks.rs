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
    let disk_type = req.disk_type.as_deref().unwrap_or("fixed");
    let mount_path = normalize_path(&req.mount_path);

    if crate::db::disks::exists_by_path(&conn, &mount_path).unwrap_or(false) {
        return HttpResponse::Conflict().json(serde_json::json!({
            "error": "Диск с таким путём уже зарегистрирован"
        }));
    }    

    match crate::db::disks::insert(&conn, &req.label, &mount_path, disk_type) {
        Ok(disk_id) => {
            if disk_type == "removable" {
                if let Err(e) = crate::storage::disks::write_marker(&mount_path, &disk_id) {
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

fn normalize_path(path: &str) -> String {
    let trimmed = path.trim_end_matches(['\\', '/']).to_string();
    
    #[cfg(target_os = "windows")]
    {
        // Windows: приводим букву диска к верхнему регистру, добавляем слэш
        if trimmed.len() == 2 && trimmed.ends_with(':') {
            return trimmed.to_uppercase() + "\\";
        }
    }
    trimmed
}

// pub async fn list_disks(
//     conn: web::Data<std::sync::Mutex<Connection>>,
// ) -> HttpResponse {
//     let conn = conn.lock().unwrap();
//     match list_disks_internal(&conn) {
//         Ok(result) => HttpResponse::Ok().json(result),
//         Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e})),
//     }
// }

pub async fn check_disks(
    conn: web::Data<std::sync::Mutex<Connection>>,
) -> HttpResponse {
    let conn = conn.lock().unwrap();

    match crate::storage::disks::sync_disks(&conn) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Проверка дисков выполнена"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

#[derive(Deserialize)]
pub struct AddRootRequest {
    pub disk_id: String,
    pub relative_path: String,
}

pub async fn add_root(
    conn: web::Data<std::sync::Mutex<Connection>>,
    req: web::Json<AddRootRequest>,
) -> HttpResponse {
    let conn = conn.lock().unwrap();

    match crate::db::disks::find_by_id(&conn, &req.disk_id) {
        Ok(Some(_)) => {}
        Ok(None) => return HttpResponse::NotFound().json(serde_json::json!({
            "error": "Диск не найден"
        })),
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }

    if crate::db::roots::exists(&conn, &req.disk_id, &req.relative_path).unwrap_or(false) {
        return HttpResponse::Conflict().json(serde_json::json!({
            "error": "Медиа-папка уже добавлена к этому диску"
        }));
    }

    match crate::db::roots::insert(&conn, &req.disk_id, &req.relative_path) {
        Ok(id) => HttpResponse::Created().json(serde_json::json!({
            "id": id,
            "message": "Медиа-папка добавлена"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub search: Option<String>,
}

pub async fn search_disks(
    conn: web::Data<std::sync::Mutex<Connection>>,
    query: web::Query<SearchQuery>,
) -> HttpResponse {
    let conn = conn.lock().unwrap();

    let search = match &query.search {
        Some(s) => s.as_str(),
        None => {
            match list_disks_internal(&conn) {
                Ok(result) => return HttpResponse::Ok().json(result),
                Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({"error": e})),
            }
        }
    };

    match crate::db::disks::search(&conn, search) {
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

fn list_disks_internal(conn: &Connection) -> Result<Vec<serde_json::Value>, String> {
    crate::db::disks::list_all(conn)
        .map(|disks| {
            disks.iter()
                .map(|d| serde_json::json!({
                    "disk_id": d.disk_id,
                    "label": d.label,
                    "mount_path": d.mount_path,
                    "disk_type": d.disk_type,
                    "is_available": d.is_available,
                }))
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[derive(Deserialize)]
pub struct RootsQuery {
    pub disk_id: String,
}

pub async fn list_roots(
    conn: web::Data<std::sync::Mutex<Connection>>,
    query: web::Query<RootsQuery>,
) -> HttpResponse {
    let conn = conn.lock().unwrap();

    match crate::db::roots::list_by_disk(&conn, &query.disk_id) {
        Ok(roots) => {
            let result: Vec<serde_json::Value> = roots
                .iter()
                .map(|r| serde_json::json!({
                    "id": r.id,
                    "disk_id": r.disk_id,
                    "relative_path": r.relative_path,
                }))
                .collect();

            HttpResponse::Ok().json(result)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

pub async fn scan_events(
    conn: web::Data<std::sync::Mutex<Connection>>,
    state: web::Data<std::sync::Mutex<crate::state::ActiveState>>,
) -> HttpResponse {
    let conn = conn.lock().unwrap();
    let state = state.lock().unwrap();

    // Получаем активный root
    let root_id = match &state.root_id {
        Some(id) => id.clone(),
        None => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Активный root не выбран. Используйте roots use"
        })),
    };

    // Получаем root из БД
    let root = match crate::db::roots::list_by_disk(&conn, &state.disk_id.clone().unwrap_or_default()) {
        Ok(roots) => roots.into_iter().find(|r| r.id == root_id),
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    };

    let root = match root {
        Some(r) => r,
        None => return HttpResponse::NotFound().json(serde_json::json!({"error": "Root не найден"})),
    };

    // Получаем диск
    let disk = match crate::db::disks::find_by_id(&conn, &root.disk_id) {
        Ok(Some(d)) => d,
        _ => return HttpResponse::NotFound().json(serde_json::json!({"error": "Диск не найден"})),
    };

    // Полный путь
    let full_path = std::path::Path::new(&disk.mount_path).join(&root.relative_path);

    match crate::services::scanner::scan_events(&conn, &root.id, &full_path.to_string_lossy()) {
        Ok(result) => HttpResponse::Ok().json(serde_json::json!({
            "total": result.total,
            "new": result.new,
            "events": result.events.iter().map(|e| serde_json::json!({
                "folder_name": e.folder_name,
                "event_date": e.event_date,
                "description": e.description,
                "is_new": e.is_new,
            })).collect::<Vec<_>>(),
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e})),
    }
}