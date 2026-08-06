use rusqlite::{Connection, Result, params};
use uuid::Uuid;

pub struct Asset {
    pub id: String,
    pub event_id: String,
    pub camera_instance_id: Option<String>,
    pub file_path: String,
    pub file_name: String,
    pub file_size: i64,
    pub media_type: String,
    pub duration_secs: Option<f64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub fps: Option<f64>,
    pub codec: Option<String>,
    pub bitrate: Option<i64>,
    pub has_audio: bool,
}

pub fn insert(
    conn: &Connection,
    event_id: &str,
    camera_instance_id: Option<&str>,
    file_path: &str,
    file_name: &str,
    file_size: i64,
    media_type: &str,
    duration_secs: Option<f64>,
    width: Option<i32>,
    height: Option<i32>,
    fps: Option<f64>,
    codec: Option<&str>,
    bitrate: Option<i64>,
    has_audio: bool,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT OR IGNORE INTO assets (id, event_id, camera_instance_id, file_path, file_name, file_size, media_type,
         duration_secs, width, height, fps, codec, bitrate, has_audio)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            id, event_id, camera_instance_id, file_path, file_name, file_size, media_type,
            duration_secs, width, height, fps, codec, bitrate, has_audio,
        ],
    )?;

    Ok(id)
}