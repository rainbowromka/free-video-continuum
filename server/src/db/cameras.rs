use rusqlite::{Connection, OptionalExtension, Result, params};
use uuid::Uuid;

pub struct Camera {
    pub id: String,
    pub name: String,
    pub folder_name: String,
}

pub fn find_or_create(conn: &Connection, name: &str, folder_name: &str) -> Result<(String, bool)> {
    // Ищем по folder_name
    if let Some(cam) = find_by_folder_name(conn, folder_name)? {
        return Ok((cam.id, false));
    }

    // Создаём новую
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO cameras (id, name, folder_name) VALUES (?1, ?2, ?3)",
        params![id, name, folder_name],
    )?;

    Ok((id, true))
}

pub fn find_by_folder_name(conn: &Connection, folder_name: &str) -> Result<Option<Camera>> {
    conn.query_row(
        "SELECT id, name, folder_name FROM cameras WHERE folder_name = ?1",
        params![folder_name],
        |row| {
            Ok(Camera {
                id: row.get(0)?,
                name: row.get(1)?,
                folder_name: row.get(2)?,
            })
        },
    )
    .optional()
}

pub fn search(conn: &Connection, query: &str) -> Result<Vec<Camera>> {
    let pattern = format!("%{}%", query);
    let mut stmt = conn.prepare(
        "SELECT id, name, folder_name FROM cameras WHERE name LIKE ?1 OR folder_name LIKE ?1 ORDER BY name"
    )?;

    let cameras = stmt.query_map(params![pattern], |row| {
        Ok(Camera {
            id: row.get(0)?,
            name: row.get(1)?,
            folder_name: row.get(2)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(cameras)
}