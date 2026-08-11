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

pub fn find_by_event(conn: &Connection, event_id: &str) -> Result<Vec<Asset>> {
    let mut stmt = conn.prepare(
        "SELECT id, event_id, camera_instance_id, file_path, file_name, file_size, media_type,
         duration_secs, width, height, fps, codec, bitrate, has_audio
         FROM assets WHERE event_id = ?1 ORDER BY file_name"
    )?;

    let assets = stmt.query_map(params![event_id], |row| {
        Ok(Asset {
            id: row.get(0)?,
            event_id: row.get(1)?,
            camera_instance_id: row.get(2)?,
            file_path: row.get(3)?,
            file_name: row.get(4)?,
            file_size: row.get(5)?,
            media_type: row.get(6)?,
            duration_secs: row.get(7)?,
            width: row.get(8)?,
            height: row.get(9)?,
            fps: row.get(10)?,
            codec: row.get(11)?,
            bitrate: row.get(12)?,
            has_audio: row.get(13)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(assets)
}