use rusqlite::{Connection, Result, params};
use uuid::Uuid;
use rusqlite::OptionalExtension;

pub struct Disk {
    pub disk_id: String,
    pub label: String,
    pub mount_path: String,
    pub disk_type: String,
    pub is_available: bool,
}

pub fn insert(conn: &Connection, label: &str, mount_path: &str, disk_type: &str) -> Result<String> {
    let disk_id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO disks (disk_id, label, mount_path, disk_type, is_available)
         VALUES (?1, ?2, ?3, ?4, 1)",
        params![disk_id, label, mount_path, disk_type],
    )?;

    Ok(disk_id)
}

pub fn find_by_path(conn: &Connection, mount_path: &str) -> Result<Option<Disk>> {
    conn.query_row(
        "SELECT disk_id, label, mount_path, disk_type, is_available
         FROM disks WHERE mount_path = ?1",
        params![mount_path],
        |row| {
            Ok(Disk {
                disk_id: row.get(0)?,
                label: row.get(1)?,
                mount_path: row.get(2)?,
                disk_type: row.get(3)?,
                is_available: row.get::<_, i32>(4)? != 0,
            })
        },
    )
    .optional()
}

// pub fn list_all, update_availability, delete — позже