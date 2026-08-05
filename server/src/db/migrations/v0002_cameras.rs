use rusqlite::Connection;
use super::Migration;

pub struct V0002Cameras;

impl Migration for V0002Cameras {
    fn version(&self) -> i32 {
        2
    }

    fn up(&self, conn: &Connection) -> rusqlite::Result<()> {
        conn.execute_batch(SQL)
    }
}

const SQL: &str = r#"
CREATE TABLE IF NOT EXISTS cameras (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    folder_name TEXT NOT NULL UNIQUE,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS camera_instances (
    id TEXT PRIMARY KEY,
    camera_id TEXT NOT NULL REFERENCES cameras(id),
    event_id TEXT NOT NULL REFERENCES events(id),
    folder_name TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(event_id, folder_name)
);
"#;