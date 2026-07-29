use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::state::ActiveState;

#[derive(Deserialize)]
pub struct SetDiskRequest {
    pub disk_id: String,
}

pub async fn set_active_disk(
    state: web::Data<std::sync::Mutex<ActiveState>>,
    req: web::Json<SetDiskRequest>,
) -> HttpResponse {
    let mut state = state.lock().unwrap();
    state.set_disk(req.disk_id.clone());
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Активный диск установлен",
        "disk_id": req.disk_id
    }))
}

#[derive(Deserialize)]
pub struct SetRootRequest {
    pub disk_id: String,
    pub root_id: String,
}

pub async fn set_active_root(
    state: web::Data<std::sync::Mutex<ActiveState>>,
    req: web::Json<SetRootRequest>,
) -> HttpResponse {
    let mut state = state.lock().unwrap();
    state.set_root(req.disk_id.clone(), req.root_id.clone());
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Активный root установлен",
        "disk_id": req.disk_id,
        "root_id": req.root_id
    }))
}

pub async fn get_active_state(
    state: web::Data<std::sync::Mutex<ActiveState>>,
) -> HttpResponse {
    let state = state.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "disk_id": state.disk_id,
        "root_id": state.root_id
    }))
}