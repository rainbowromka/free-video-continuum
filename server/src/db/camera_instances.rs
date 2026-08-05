use rusqlite::{Connection, OptionalExtension, Result,  params};
use uuid::Uuid;

pub struct CameraInstance {
    pub id: String,
    pub camera_id: String,
    pub event_id: String,
    pub folder_name: String,
}

pub fn find_or_create(conn: &Connection, camera_id: &str, event_id: &str, folder_name: &str) -> Result<(String, bool)> {
    // Ищем существующий
    if let Some(inst) = find_by_event_and_folder(conn, event_id, folder_name)? {
        return Ok((inst.id, false));
    }

    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO camera_instances (id, camera_id, event_id, folder_name) VALUES (?1, ?2, ?3, ?4)",
        params![id, camera_id, event_id, folder_name],
    )?;

    Ok((id, true))
}

pub fn find_by_event_and_folder(conn: &Connection, event_id: &str, folder_name: &str) -> Result<Option<CameraInstance>> {
    conn.query_row(
        "SELECT id, camera_id, event_id, folder_name FROM camera_instances WHERE event_id = ?1 AND folder_name = ?2",
        params![event_id, folder_name],
        |row| {
            Ok(CameraInstance {
                id: row.get(0)?,
                camera_id: row.get(1)?,
                event_id: row.get(2)?,
                folder_name: row.get(3)?,
            })
        },
    )
    .optional()
}