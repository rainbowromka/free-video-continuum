use dialoguer::{Confirm, Input, Select};
use std::env;
use std::path::{Path, PathBuf};

pub async fn add_disk_wizard() {
    println!("=== Мастер добавления диска ===\n");

    // Шаг 1: определяем путь
    let current_dir = env::current_dir().unwrap_or_default();

    // Проверяем, не зарегистрирована ли уже текущая папка
    if crate::client::find_disk_by_path(&current_dir.to_string_lossy()).await.is_some() {
        eprintln!("[ERROR] Текущая папка '{}' уже зарегистрирована как диск", current_dir.display());
        return;
    }

    println!("Текущая папка: {}", current_dir.display());
    let use_current = Confirm::new()
        .with_prompt("Использовать эту папку?")
        .default(true)
        .interact()
        .unwrap();

    let mount_path = if use_current {
        current_dir.to_string_lossy().to_string()
    } else {
        Input::<String>::new()
            .with_prompt("Введите путь к диску или папке")
            .interact_text()
            .unwrap()
    };

    // Проверяем, не зарегистрирован ли уже этот путь
    if crate::client::find_disk_by_path(&mount_path).await.is_some() {
        eprintln!("[ERROR] Диск с путём '{}' уже зарегистрирован", mount_path);
        return;
    }

    // Шаг 2: название диска
    let default_label = std::path::Path::new(&mount_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Disk".to_string());

    let label: String = Input::new()
        .with_prompt("Название диска")
        .default(default_label)
        .interact_text()
        .unwrap();

    // Шаг 3: тип диска
    let disk_types = vec!["fixed", "removable"];
    let default_type = if mount_path.ends_with(':') || mount_path.starts_with("/mnt/") {
        1 // removable
    } else {
        0 // fixed
    };

    let type_idx = Select::new()
        .with_prompt("Тип диска")
        .items(&disk_types)
        .default(default_type)
        .interact()
        .unwrap();
    let disk_type = disk_types[type_idx];

    // Шаг 4: подтверждение и отправка
    println!("\nИтого:");
    println!("  Путь:   {}", mount_path);
    println!("  Название: {}", label);
    println!("  Тип:    {}", disk_type);

    let confirm = Confirm::new()
        .with_prompt("Зарегистрировать диск?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        match crate::client::add_disk(&label, &mount_path, disk_type).await {
            Ok(resp) => println!("[OK] Диск зарегистрирован: {}", resp.disk_id),
            Err(e) => eprintln!("[ERROR] {}", e),
        }
    } else {
        println!("Отмена.");
    }
}

fn find_disk_by_marker(current_dir: &Path) -> Option<(String, PathBuf)> {
    let mut dir = current_dir.to_path_buf();
    
    loop {
        let marker = dir.join(".continuum-disk");
        if marker.exists() {
            // Читаем disk_id из маркера
            let content = std::fs::read_to_string(&marker).ok()?;
            let parsed: serde_json::Value = serde_json::from_str(&content).ok()?;
            let disk_id = parsed.get("disk_id")?.as_str()?.to_string();
            return Some((disk_id, dir));
        }
        
        // Поднимаемся на уровень выше
        if !dir.pop() {
            // Достигли корня
            break;
        }
    }
    
    None
}

pub async fn add_root_wizard() {
    println!("=== Мастер добавления медиа-папки ===\n");

    let current_dir = env::current_dir().unwrap_or_default();
    println!("Текущая папка: {}", current_dir.display());

    // Шаг 1: ищем диск по маркеру
    if let Some((disk_id, mount_path)) = find_disk_by_marker(&current_dir) {
        println!("Найден диск: {}", mount_path.display());
        
        // Определяем relative_path
        let relative_path = current_dir
            .strip_prefix(&mount_path)
            .unwrap_or(&current_dir)
            .to_string_lossy()
            .to_string();
        
        if relative_path.is_empty() {
            eprintln!("[ERROR] Нельзя добавить корень диска как медиа-папку");
            return;
        }
        
        println!("Относительный путь: {}", relative_path);
        
        // TODO: проверка дубликата и отправка
        match crate::client::add_root(&disk_id, &relative_path).await {
            Ok(msg) => println!("[OK] {}", msg),
            Err(e) => eprintln!("[ERROR] {}", e),
        }
        return;
    }

    // Шаг 2: не нашли маркер — ищем среди fixed дисков
    match crate::client::list_disks().await {
        Ok(disks) => {
            let /*mut*/ candidates: Vec<_> = disks
                .iter()
                .filter(|d| d.disk_type == "fixed" && d.is_available)
                .filter(|d| current_dir.to_string_lossy().starts_with(&d.mount_path))
                .collect();

            if candidates.is_empty() {
                eprintln!("[ERROR] Диск не найден. Зарегистрируйте диск сначала (continuum add)");
                return;
            }

            if candidates.len() == 1 {
                let disk = candidates[0];
                let relative_path = current_dir
                    .strip_prefix(&disk.mount_path)
                    .unwrap_or(&current_dir)
                    .to_string_lossy()
                    .to_string();
                
                if relative_path.is_empty() {
                    eprintln!("[ERROR] Нельзя добавить корень диска как медиа-папку");
                    return;
                }

                println!("Найден диск: {} ({})", disk.label, disk.disk_id);
                println!("Относительный путь: {}", relative_path);

                match crate::client::add_root(&disk.disk_id, &relative_path).await {
                    Ok(msg) => println!("[OK] {}", msg),
                    Err(e) => eprintln!("[ERROR] {}", e),
                }
            } else {
                println!("Найдено несколько подходящих дисков:");
                for d in &candidates {
                    println!("  {} - {} ({})", d.disk_id, d.label, d.mount_path);
                }
                eprintln!("[ERROR] Уточните диск командой: continuum roots add <contains> <path>");
            }
        }
        Err(e) => eprintln!("[ERROR] {}", e),
    }
}


/// Интерактивно разрешает камеру по folder_name.
/// Возвращает camera_id (новый или существующий).
pub async fn resolve_camera(folder_name: &str) -> Result<String, String> {
    println!("\nНайдена папка камеры: '{}'", folder_name);

    // Ищем существующую камеру по folder_name
    let existing = crate::client::search_cameras(folder_name).await?;
    
    if !existing.is_empty() {
        // Показываем найденные
        println!("Найдены похожие камеры:");
        for (i, cam) in existing.iter().enumerate() {
            let name = cam["name"].as_str().unwrap_or("");
            let fnm = cam["folder_name"].as_str().unwrap_or("");
            println!("  {}. {} (папка: {})", i + 1, name, fnm);
        }
        println!("  0. Создать новую камеру");
        
        let selection = Select::new()
            .with_prompt("Выберите камеру")
            .default(0)
            .items(&{
                let mut items: Vec<String> = existing.iter()
                    .map(|c| c["name"].as_str().unwrap_or("").to_string())
                    .collect();
                items.push("Новая камера".to_string());
                items
            })
            .interact()
            .unwrap();

        if selection < existing.len() {
            let camera_id = existing[selection]["id"].as_str().unwrap().to_string();
            
            // Создаём/находим camera_instance
            let name = existing[selection]["name"].as_str().unwrap_or(folder_name);
            let resp = crate::client::create_camera(name, folder_name).await?;
            println!("[OK] Использована камера: {}", name);
            return Ok(camera_id);
        }
    }

    // Новая камера
    let name: String = Input::new()
        .with_prompt("Введите имя камеры")
        .default(folder_name.to_string())
        .interact_text()
        .unwrap();

    let resp = crate::client::create_camera(&name, folder_name).await?;
    // Парсим ответ чтобы получить id
    let parsed: serde_json::Value = serde_json::from_str(&resp).unwrap_or_default();
    let camera_id = parsed["id"].as_str().unwrap_or("").to_string();
    
    println!("[OK] Создана новая камера: {}", name);
    Ok(camera_id)
}