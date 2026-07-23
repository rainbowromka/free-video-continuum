use rusqlite::{Connection, Result, params};
use uuid::Uuid;

pub struct Root {
    pub id: String,
    pub disk_id: String,
    pub relative_path: String,
}

pub fn insert(conn: &Connection, disk_id: &str, relative_path: &str) -> Result<String> {
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO roots (id, disk_id, relative_path)
         VALUES (?1, ?2, ?3)",
        params![id, disk_id, relative_path],
    )?;

    Ok(id)
}

pub fn list_by_disk(conn: &Connection, disk_id: &str) -> Result<Vec<Root>> {
    let mut stmt = conn.prepare(
        "SELECT id, disk_id, relative_path
         FROM roots WHERE disk_id = ?1
         ORDER BY relative_path"
    )?;

    let roots = stmt.query_map(params![disk_id], |row| {
        Ok(Root {
            id: row.get(0)?,
            disk_id: row.get(1)?,
            relative_path: row.get(2)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(roots)
}