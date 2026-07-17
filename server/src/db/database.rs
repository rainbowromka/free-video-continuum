use rusqlite::{Connection, Result};
use crate::db::migrations::{Migration, v0001_create_tables};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL")?;
        let db = Self { conn };
        db.ensure_schema_version_table()?;
        db.migrate()?;
        Ok(db)
    }

    fn ensure_schema_version_table(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY,
                applied_at TEXT DEFAULT (datetime('now'))
            )"
        )?;
        Ok(())
    }

    fn current_version(&self) -> Result<i32> {
        let version: i32 = self.conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_version",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);
        Ok(version)
    }

    fn migrate(&self) -> Result<()> {
        let from = self.current_version()?;

        let migrations: Vec<Box<dyn Migration>> = vec![
            Box::new(v0001_create_tables::V0001CreateTables),
        ];

        let current_version = migrations.iter()
            .map(|m| m.version())
            .max()
            .unwrap_or(0);

        for migration in &migrations {
            if from < migration.version() {
                migration.up(&self.conn)?;
            }
        }

        self.conn.execute(
            "INSERT OR REPLACE INTO schema_version (version) VALUES (?1)",
            [current_version],
        )?;
        Ok(())
    }
}