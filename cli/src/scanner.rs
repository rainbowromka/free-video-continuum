use std::path::Path;
use std::process::Command;

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

pub struct VideoFile {
    pub file_path: String,
    pub file_name: String,
    pub file_size: i64,
    pub media_type: String,
    pub duration_secs: Option<f64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub fps: Option<f64>,
    pub codec: Option<String>,
    pub bitrate: Option<i64>,
    pub has_audio: bool,
}

/// Сканирует папку, находит видеофайлы, извлекает метаданные через ffprobe
pub fn scan_videos_in_folder(folder_path: &str) -> Result<Vec<VideoFile>, String> {
    let mut videos = Vec::new();
    let video_extensions = ["mp4", "mov", "avi", "mkv", "mts", "m2ts", "webm", "insv", "lrv", "360"];

    scan_recursive(Path::new(folder_path), &video_extensions, &mut videos)?;
    Ok(videos)
}

fn scan_recursive(dir: &Path, extensions: &[&str], videos: &mut Vec<VideoFile>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("Ошибка чтения папки {}: {}", dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Ошибка: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            scan_recursive(&path, extensions, videos)?;
        } else if path.is_file() {
            let ext = path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            if extensions.contains(&ext.as_str()) {
                let file_name = path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let file_size = path.metadata()
                    .map(|m| m.len() as i64)
                    .unwrap_or(0);

                let media_type = match ext.as_str() {
                    "mp4" | "mov" | "avi" | "mkv" | "mts" | "m2ts" | "webm" | "insv" => "video",
                    "lrv" => "video", // low-res video
                    "360" => "video", // 360 video
                    _ => "video",
                };

                // Извлекаем метаданные через ffprobe
                let metadata = extract_metadata(&path);

                videos.push(VideoFile {
                    file_path: path.to_string_lossy().to_string(),
                    file_name,
                    file_size,
                    media_type: media_type.to_string(),
                    duration_secs: metadata.duration,
                    width: metadata.width,
                    height: metadata.height,
                    fps: metadata.fps,
                    codec: metadata.codec,
                    bitrate: metadata.bitrate,
                    has_audio: metadata.has_audio,
                });
            }
        }
    }

    Ok(())
}

struct VideoMetadata {
    duration: Option<f64>,
    width: Option<i32>,
    height: Option<i32>,
    fps: Option<f64>,
    codec: Option<String>,
    bitrate: Option<i64>,
    has_audio: bool,
}

fn extract_metadata(file_path: &Path) -> VideoMetadata {
    let output = Command::new("ffprobe")
        .args([
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            &file_path.to_string_lossy(),
        ])
        .output();

    let mut metadata = VideoMetadata {
        duration: None,
        width: None,
        height: None,
        fps: None,
        codec: None,
        bitrate: None,
        has_audio: false,
    };

    if let Ok(output) = output {
        if output.status.success() {
            if let Ok(parsed) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
                // Парсим streams
                if let Some(streams) = parsed["streams"].as_array() {
                    for stream in streams {
                        let codec_type = stream["codec_type"].as_str().unwrap_or("");
                        match codec_type {
                            "video" => {
                                metadata.width = stream["width"].as_i64().map(|v| v as i32);
                                metadata.height = stream["height"].as_i64().map(|v| v as i32);
                                metadata.codec = stream["codec_name"].as_str().map(|s| s.to_string());

                                // FPS
                                if let Some(fps_str) = stream["r_frame_rate"].as_str() {
                                    let parts: Vec<&str> = fps_str.split('/').collect();
                                    if parts.len() == 2 {
                                        if let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                                            if den != 0.0 {
                                                metadata.fps = Some(num / den);
                                            }
                                        }
                                    }
                                }
                            }
                            "audio" => {
                                metadata.has_audio = true;
                            }
                            _ => {}
                        }
                    }
                }

                // Парсим format
                let format = &parsed["format"];
                if !format.is_null() {
                    if let Some(dur) = format["duration"].as_str() {
                        metadata.duration = dur.parse().ok();
                    }
                    if let Some(br) = format["bit_rate"].as_str() {
                        metadata.bitrate = br.parse().ok();
                    }
                }
            }
        }
    }

    metadata
}