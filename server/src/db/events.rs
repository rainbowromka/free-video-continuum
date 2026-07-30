use rusqlite::{Connection, Result, params};
use uuid::Uuid;

pub struct Event {
    pub id: String,
    pub media_root_id: String,
    pub folder_name: String,
    pub event_date: Option<String>,
    pub description: Option<String>,
}

pub fn insert(
    conn: &Connection,
    media_root_id: &str,
    folder_name: &str,
    event_date: Option<&str>,
    description: Option<&str>,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT OR IGNORE INTO events (id, media_root_id, folder_name, event_date, description)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, media_root_id, folder_name, event_date, description],
    )?;

    // Если запись уже была (IGNORE), получаем существующий id
    if conn.changes() == 0 {
        let existing_id: String = conn.query_row(
            "SELECT id FROM events WHERE media_root_id = ?1 AND folder_name = ?2",
            params![media_root_id, folder_name],
            |row| row.get(0),
        )?;
        return Ok(existing_id);
    }

    Ok(id)
}

pub fn find_by_root(conn: &Connection, media_root_id: &str) -> Result<Vec<Event>> {
    let mut stmt = conn.prepare(
        "SELECT id, media_root_id, folder_name, event_date, description
         FROM events WHERE media_root_id = ?1
         ORDER BY folder_name"
    )?;

    let events = stmt.query_map(params![media_root_id], |row| {
        Ok(Event {
            id: row.get(0)?,
            media_root_id: row.get(1)?,
            folder_name: row.get(2)?,
            event_date: row.get(3)?,
            description: row.get(4)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(events)
}