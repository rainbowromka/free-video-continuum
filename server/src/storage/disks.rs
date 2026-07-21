use rusqlite::Connection;
use std::path::Path;
use serde::{Deserialize, Serialize};

const MARKER_FILENAME: &str = ".continuum-disk";

#[derive(Serialize, Deserialize)]
struct DiskMarker {
    disk_id: String,
}

/// Создать файл .continuum-disk в корне диска
pub fn write_marker(mount_path: &str, disk_id: &str) -> Result<(), String> {
    let marker_path = Path::new(mount_path).join(MARKER_FILENAME);
    let marker = DiskMarker {
        disk_id: disk_id.to_string(),
    };
    let json = serde_json::to_string_pretty(&marker)
        .map_err(|e| format!("Ошибка сериализации: {}", e))?;
    std::fs::write(&marker_path, json)
        .map_err(|e| format!("Ошибка записи {}: {}", marker_path.display(), e))?;
    Ok(())
}

/// Прочитать disk_id из .continuum-disk
pub fn read_marker(mount_path: &str) -> Result<String, String> {
    let marker_path = Path::new(mount_path).join(MARKER_FILENAME);
    let json = std::fs::read_to_string(&marker_path)
        .map_err(|e| format!("Ошибка чтения {}: {}", marker_path.display(), e))?;
    let marker: DiskMarker = serde_json::from_str(&json)
        .map_err(|e| format!("Ошибка парсинга: {}", e))?;
    Ok(marker.disk_id)
}

/// Проверить, есть ли .continuum-disk в корне
pub fn has_marker(mount_path: &str) -> bool {
    Path::new(mount_path).join(MARKER_FILENAME).exists()
}

/// Обойти буквы дисков (Windows) и найти все .continuum-disk
#[cfg(target_os = "windows")]
pub fn scan_physical_disks() -> Vec<(String, String)> {
    let mut found = Vec::new();
    for letter in 'A'..='Z' {
        let path = format!("{}:\\", letter);
        if Path::new(&path).exists() {
            if let Ok(disk_id) = read_marker(&path) {
                found.push((disk_id, path));
            }
        }
    }
    found
}

/// Обойти точки монтирования (Linux) и найти все .continuum-disk
#[cfg(target_os = "linux")]
pub fn scan_physical_disks() -> Vec<(String, String)> {
    let mut found = Vec::new();
    let search_paths = vec!["/mnt", "/media"];
    for base in search_paths {
        if let Ok(entries) = std::fs::read_dir(base) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let path_str = path.to_string_lossy().to_string();
                    if let Ok(disk_id) = read_marker(&path_str) {
                        found.push((disk_id, path_str));
                    }
                }
            }
        }
    }
    found
}

/// Синхронизировать состояние дисков: сравнить физически найденные с БД
pub fn sync_disks(conn: &Connection) -> Result<(), String> {
    let found = scan_physical_disks(); // Vec<(disk_id, mount_path)>

    // Получаем все removable диски из БД
    let db_disks = crate::db::disks::list_all(conn)
        .map_err(|e| format!("Ошибка чтения дисков из БД: {}", e))?;

    for db_disk in db_disks {
        match db_disk.disk_type.as_str() {
            "removable" => {
                // Ищем маркер на дисках
                if let Some((_, new_path)) = found.iter().find(|(id, _)| *id == db_disk.disk_id) {
                    // Диск найден — обновляем путь и делаем доступным
                    crate::db::disks::update_availability(conn, &db_disk.disk_id, true, Some(new_path))
                        .map_err(|e| format!("Ошибка обновления диска: {}", e))?;
                } else {
                    // Диск не найден — помечаем недоступным
                    crate::db::disks::update_availability(conn, &db_disk.disk_id, false, None)
                        .map_err(|e| format!("Ошибка обновления диска: {}", e))?;
                }
            }
            "fixed" => {
                // Для fixed дисков просто проверяем существование пути
                let available = std::path::Path::new(&db_disk.mount_path).exists();
                crate::db::disks::update_availability(conn, &db_disk.disk_id, available, None)
                    .map_err(|e| format!("Ошибка обновления диска: {}", e))?;
            }
            _ => {}
        }
    }

    Ok(())
}