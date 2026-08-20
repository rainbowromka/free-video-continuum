use serde::{Deserialize, Serialize};

const SERVER_URL: &str = "http://127.0.0.1:9090";

fn server_url() -> String {
    SERVER_URL.to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiskInfo {
    pub disk_id: String,
    pub label: String,
    pub mount_path: String,
    pub disk_type: String,
    pub is_available: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RootInfo {
    pub id: String,
    pub disk_id: String,
    pub relative_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventInfo {
    pub id: String,
    pub folder_name: String,
    pub event_date: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetInfo {
    pub id: String,
    pub file_name: String,
    pub media_type: String,
    pub camera_instance_id: Option<String>,
    pub duration_secs: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CameraInstanceInfo {
    pub id: String,
    pub camera_name: String,
    pub folder_name: String,
}

fn get_json<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    let response = ureq::get(url).call().map_err(|e| e.to_string())?;
    if response.status() == 200 {
        let mut body = response.into_body();
        let text = body.read_to_string().map_err(|e| e.to_string())?;
        serde_json::from_str(&text).map_err(|e| e.to_string())
    } else {
        Err(format!("Ошибка сервера: {}", response.status()))
    }
}

pub fn fetch_disks() -> Result<Vec<DiskInfo>, String> {
    get_json(&format!("{}/api/admin/disks", server_url()))
}

pub fn fetch_roots(disk_id: &str) -> Result<Vec<RootInfo>, String> {
    get_json(&format!("{}/api/admin/roots?disk_id={}", server_url(), disk_id))
}

pub fn fetch_events(root_id: &str) -> Result<Vec<EventInfo>, String> {
    get_json(&format!("{}/api/admin/roots/{}/events", server_url(), root_id))
}

pub fn fetch_assets(event_id: &str) -> Result<Vec<AssetInfo>, String> {
    get_json(&format!("{}/api/admin/events/{}/assets", server_url(), event_id))
}

pub fn fetch_camera_instance(id: &str) -> Result<CameraInstanceInfo, String> {
    get_json(&format!("{}/api/admin/camera-instances/{}", server_url(), id))
}

pub fn fetch_cameras(event_id: &str) -> Result<Vec<CameraInstanceInfo>, String> {
    get_json(&format!("{}/api/admin/events/{}/camera-instances", server_url(), event_id))
}