use rusqlite::Connection;
use super::Migration;

pub struct V0001CreateTables;

impl Migration for V0001CreateTables {
    fn version(&self) -> i32 {
        1
    }

    fn up(&self, conn: &Connection) -> rusqlite::Result<()> {
        conn.execute_batch(SQL)
    }
}

const SQL: &str = r#"
CREATE TABLE IF NOT EXISTS disks (
    disk_id TEXT PRIMARY KEY,
    label TEXT NOT NULL,
    mount_path TEXT,
    disk_type TEXT DEFAULT 'fixed',
    is_available INTEGER DEFAULT 0,
    last_seen_at TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS roots (
    id TEXT PRIMARY KEY,
    disk_id TEXT NOT NULL REFERENCES disks(disk_id),
    relative_path TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY,
    media_root_id TEXT NOT NULL REFERENCES media_roots(id),
    folder_name TEXT NOT NULL,
    event_date TEXT,
    description TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS assets (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id),
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER,
    media_type TEXT,
    source TEXT,
    duration_secs REAL,
    width INTEGER,
    height INTEGER,
    fps REAL,
    codec TEXT,
    bitrate INTEGER,
    has_audio INTEGER DEFAULT 0,
    is_usable INTEGER DEFAULT 1,
    notes TEXT,
    rating INTEGER DEFAULT 0,
    is_processed INTEGER DEFAULT 0,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS asset_tags (
    asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    tag_id TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (asset_id, tag_id)
);

CREATE TABLE IF NOT EXISTS subclips (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    start_time REAL NOT NULL,
    end_time REAL NOT NULL,
    description TEXT,
    notes TEXT,
    rating INTEGER DEFAULT 0,
    is_usable INTEGER DEFAULT 1,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS subclip_tags (
    subclip_id TEXT NOT NULL REFERENCES subclips(id) ON DELETE CASCADE,
    tag_id TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (subclip_id, tag_id)
);

CREATE TABLE IF NOT EXISTS proxies (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    resolution TEXT NOT NULL,
    file_path TEXT,
    file_size INTEGER,
    status TEXT DEFAULT 'pending',
    progress REAL DEFAULT 0,
    error_message TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS comparisons (
    id TEXT PRIMARY KEY,
    asset_a_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    asset_b_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    tag_id TEXT REFERENCES tags(id),
    winner TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS asset_scores (
    asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    tag_id TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    elo_score REAL DEFAULT 1500,
    technical_bonus REAL DEFAULT 0,
    comparisons_count INTEGER DEFAULT 0,
    PRIMARY KEY (asset_id, tag_id)
);

CREATE TABLE IF NOT EXISTS catalog_progress (
    asset_id TEXT PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    status TEXT DEFAULT 'pending',
    reviewed_at TEXT
);
"#;