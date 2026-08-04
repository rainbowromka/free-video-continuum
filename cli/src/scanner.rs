use std::path::Path;

/// Результат сканирования папки событий
pub struct ScanEventsResult {
    pub total: usize,
    pub new: usize,
    pub events: Vec<EventFolder>,
}

pub struct EventFolder {
    pub folder_name: String,
    pub event_date: Option<String>,
    pub description: Option<String>,
    pub full_path: String,
}

/// Обходит папки первого уровня внутри root_path и возвращает список событий
pub fn find_events(root_path: &str) -> Result<ScanEventsResult, String> {
    let root = Path::new(root_path);

    if !root.exists() || !root.is_dir() {
        return Err(format!("Папка не существует: {}", root_path));
    }

    let mut result = ScanEventsResult {
        total: 0,
        new: 0,
        events: Vec::new(),
    };

    let entries = std::fs::read_dir(root)
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

        let (event_date, description) = parse_event_folder(&folder_name);

        result.total += 1;
        result.events.push(EventFolder {
            folder_name,
            event_date,
            description,
            full_path: path.to_string_lossy().to_string(),
        });
    }

    Ok(result)
}

/// Парсит имя папки: "20260601 Субботник" → ("2026-06-01", "Субботник")
fn parse_event_folder(name: &str) -> (Option<String>, Option<String>) {
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