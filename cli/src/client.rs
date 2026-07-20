use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

const DEFAULT_SERVER_URL: &str = "http://127.0.0.1:9090";

#[derive(Serialize)]
struct AddDiskRequest {
    label: String,
    mount_path: String,
    disk_type: String,
}

#[derive(Deserialize)]
pub struct AddDiskResponse {
    pub disk_id: String,
    pub message: String,
}

fn server_url() -> String {
      // TODO: в будущем читать из файла конфигурации
    DEFAULT_SERVER_URL.to_string()
}

pub async fn add_disk(label: &str, mount_path: &str, disk_type: &str) -> Result<AddDiskResponse, String> {
    let url = format!("{}/api/admin/disks", server_url());
    let client = Client::new();
    let req = AddDiskRequest {
        label: label.to_string(),
        mount_path: mount_path.to_string(),
        disk_type: disk_type.to_string(),
    };

    let response = client
        .post(&url)
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Ошибка подключения к серверу: {}", e))?;

    if response.status().is_success() {
        response
            .json::<AddDiskResponse>()
            .await
            .map_err(|e| format!("Ошибка чтения ответа: {}", e))
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!("Ошибка сервера ({}): {}", status, body))
    }
}

pub async fn health_check() -> Result<String, String> {
    let url = format!("{}/health", server_url());
    let client = Client::new();

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Сервер недоступен: {}", e))?;

    response
        .text()
        .await
        .map_err(|e| format!("Ошибка чтения ответа: {}", e))
}

#[derive(Deserialize)]
pub struct DiskInfo {
    pub disk_id: String,
    pub label: String,
    pub mount_path: String,
    pub disk_type: String,
    pub is_available: bool,
}

pub async fn list_disks() -> Result<Vec<DiskInfo>, String> {
    let url = format!("{}/api/admin/disks", server_url());
    let client = Client::new();

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Ошибка подключения к серверу: {}", e))?;

    if response.status().is_success() {
        response
            .json::<Vec<DiskInfo>>()
            .await
            .map_err(|e| format!("Ошибка чтения ответа: {}", e))
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!("Ошибка сервера ({}): {}", status, body))
    }
}