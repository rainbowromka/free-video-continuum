use rusqlite::Connection;

pub mod v0001_create_tables;

pub trait Migration {
    fn version(&self) -> i32;
    fn up(&self, conn: &Connection) -> rusqlite::Result<()>;
}