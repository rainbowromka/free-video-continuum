use rusqlite::Connection;
use std::path::Path;

pub struct ScanEventsResult {
    pub total: usize,
    pub new: usize,
    pub events: Vec<EventInfo>,
}

pub struct EventInfo {
    pub folder_name: String,
    pub event_date: Option<String>,
    pub description: Option<String>,
    pub is_new: bool,
}

/// Обходит папки первого уровня внутри root и записывает события
pub fn scan_events(conn: &Connection, media_root_id: &str, full_path: &str) -> Result<ScanEventsResult, String> {
    let root_path = Path::new(full_path);
    
    if !root_path.exists() || !root_path.is_dir() {
        return Err(format!("Папка не существует: {}", full_path));
    }

    let mut result = ScanEventsResult {
        total: 0,
        new: 0,
        events: Vec::new(),
    };

    let entries = std::fs::read_dir(root_path)
        .map_err(|e| format!("Ошибка чтения папки: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Ошибка чтения: {}", e))?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let folder_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Парсим имя папки: yyyyMMdd описание
        let (event_date, description) = parse_event_folder(&folder_name);

        result.total += 1;

        match crate::db::events::insert(
            conn,
            media_root_id,
            &folder_name,
            event_date.as_deref(),
            description.as_deref(),
        ) {
            Ok(id) => {
                let is_new = id.len() > 0; // упрощённо
                if is_new {
                    result.new += 1;
                }
                result.events.push(EventInfo {
                    folder_name,
                    event_date,
                    description,
                    is_new,
                });
            }
            Err(e) => {
                eprintln!("Ошибка записи события {}: {}", folder_name, e);
            }
        }
    }

    Ok(result)
}

fn parse_event_folder(name: &str) -> (Option<String>, Option<String>) {
    // Проверяем, начинается ли с 8 цифр
    let chars: Vec<char> = name.chars().collect();
    if chars.len() >= 8 && chars[..8].iter().all(|c| c.is_ascii_digit()) {
        let date_str: String = chars[..8].iter().collect();
        let desc: String = chars[8..].iter().collect();
        let desc = desc.trim().to_string();
        
        let formatted_date = format!(
            "{}-{}-{}",
            &date_str[..4],
            &date_str[4..6],
            &date_str[6..8]
        );
        
        let desc = if desc.is_empty() { None } else { Some(desc) };
        (Some(formatted_date), desc)
    } else {
        (None, Some(name.to_string()))
    }
}