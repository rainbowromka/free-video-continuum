use crate::client;
use crate::display;
use std::path::Path;
use dialoguer::Confirm;

pub async fn handle_add(contains: Option<String>, path: Option<String>) {
    match (contains, path) {
        (Some(contains), Some(path)) => {
            match client::search_disks(&contains).await {
                Ok(disks) if disks.len() == 1 => {
                    let disk = &disks[0];
                    match client::add_root(&disk.disk_id, &path).await {
                        Ok(msg) => println!("[OK] {}", msg),
                        Err(e) => eprintln!("[ERROR] {}", e),
                    }
                }
                Ok(disks) if disks.is_empty() => {
                    eprintln!("[ERROR] Диск не найден по '{}'", contains);
                }
                Ok(disks) => {
                    println!("Найдено несколько дисков:");
                    for d in &disks {
                        println!("  {} - {}", d.disk_id, d.label);
                    }
                    eprintln!("[ERROR] Уточните запрос");
                }
                Err(e) => eprintln!("[ERROR] {}", e),
            }
        }
        _ => {
            crate::interactive::add_root_wizard().await;
        }
    }
}

pub async fn handle_ls(contains: Option<String>) {
    let active = client::get_active_state().await.ok();
    let active_root_id = active.as_ref().and_then(|s| s.root_id.as_deref());

    let disk_id = match contains {
        Some(ref c) => {
            match client::search_disks(c).await {
                Ok(disks) if disks.len() == 1 => Some(disks[0].disk_id.clone()),
                Ok(disks) if disks.is_empty() => {
                    eprintln!("[ERROR] Диск не найден по '{}'", c);
                    return;
                }
                Ok(disks) => {
                    println!("Найдено несколько дисков, уточните:");
                    for d in &disks {
                        println!("  {} - {}", d.disk_id, d.label);
                    }
                    return;
                }
                Err(e) => {
                    eprintln!("[ERROR] {}", e);
                    return;
                }
            }
        }
        None => active.as_ref().and_then(|s| s.disk_id.clone()),
    };

    match disk_id {
        Some(id) => {
            match client::list_roots(&id).await {
                Ok(roots) => display::print_roots(&roots, active_root_id),
                Err(e) => eprintln!("[ERROR] {}", e),
            }
        }
        None => {
            eprintln!("[ERROR] Диск не выбран. Используйте disk use или укажите contains");
        }
    }
}

pub async fn handle_use(contains: &str) {
    let state = match client::get_active_state().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            return;
        }
    };
    let disk_id = match state.disk_id {
        Some(id) => id,
        None => {
            eprintln!("[ERROR] Сначала выберите диск: continuum disk use <contains>");
            return;
        }
    };
    match client::list_roots(&disk_id).await {
        Ok(roots) => {
            let matched: Vec<_> = roots.iter().filter(|r| r.relative_path.contains(contains)).collect();
            if matched.len() == 1 {
                let root = matched[0];
                match client::set_active_root(&disk_id, &root.id).await {
                    Ok(_) => println!("[OK] Активный root: {}", root.relative_path),
                    Err(e) => eprintln!("[ERROR] {}", e),
                }
            } else if matched.is_empty() {
                eprintln!("[ERROR] Root не найден");
            } else {
                println!("Найдено несколько root'ов, уточните:");
                for r in &matched {
                    println!("  {} - {}", r.id, r.relative_path);
                }
            }
        }
        Err(e) => eprintln!("[ERROR] {}", e),
    }
}

pub async fn handle_scan() {
    // Получаем активный root через сервер
    let state = match client::get_active_state().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            return;
        }
    };

    let (disk_id, root_id) = match (&state.disk_id, &state.root_id) {
        (Some(d), Some(r)) => (d.clone(), r.clone()),
        _ => {
            eprintln!("[ERROR] Активный диск или root не выбран");
            return;
        }
    };

    // Получаем информацию о диске и root'е
    let disk = match client::search_disks(&disk_id).await {
        Ok(disks) if disks.len() == 1 => disks[0].clone(),
        _ => {
            eprintln!("[ERROR] Диск не найден");
            return;
        }
    };

    let roots = match client::list_roots(&disk_id).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            return;
        }
    };

    let root = match roots.iter().find(|r| r.id == root_id) {
        Some(r) => r,
        None => {
            eprintln!("[ERROR] Root не найден");
            return;
        }
    };

    // Полный путь
    let full_path = Path::new(&disk.mount_path).join(&root.relative_path);
    let full_path_str = full_path.to_string_lossy().to_string();

    println!("Сканирование: {}", full_path_str);

    // Ищем события
    let scan_result = match crate::scanner::find_events(&full_path_str) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            return;
        }
    };

    println!("Найдено папок: {}", scan_result.total);

    for event in &scan_result.events {

        let scan_event = Confirm::new()
            .with_prompt(format!("  Сканировать событие '{}'?", event.folder_name))
            .default(true)
            .interact()
            .unwrap_or(true);

        if !scan_event {
            println!("    [SKIP]");
            continue;
        }

        match client::create_event(&root_id, &event.folder_name, event.event_date.as_deref(), event.description.as_deref()).await {
            Ok(response) => {
                if response.contains("\"is_new\":true") {
                    println!("  [ADDED] {} - OK", event.folder_name);
                } else {
                    println!("  [EXIST] {} - OK", event.folder_name);
                }

                let event_id = get_id_from_response(&response);
                if let Some(id) = event_id {
                    scan_event_cameras(&event.full_path, &id, &event.folder_name).await;
                }
            }
            Err(e) => eprintln!("  [ERROR] {}: {}", event.folder_name, e),
        }
    }

    fn get_id_from_response(response: &str) -> Option<String> {
        let parsed: serde_json::Value = serde_json::from_str(response).ok()?;
        parsed["id"].as_str().map(|s| s.to_string())
    }

    async fn scan_event_cameras(event_path: &str, event_id: &str, event_name: &str) {
        let subdirs = match std::fs::read_dir(event_path) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .collect::<Vec<_>>(),
            Err(_) => return,
        };

        if subdirs.is_empty() {
            return;
        }

        println!("    Камеры в событии '{}':", event_name);
        for subdir in &subdirs {
            let folder_name = subdir.file_name().to_string_lossy().to_string();

            let scan_camera = Confirm::new()
                .with_prompt(format!("      Сканировать камеру '{}'?", folder_name))
                .default(true)
                .interact()
                .unwrap_or(true);

            if !scan_camera {
                println!("      [SKIP] {}", folder_name);
                continue;
            }
           
            let folder_name = subdir.file_name().to_string_lossy().to_string();
            print!("      {} ... ", folder_name);

            match crate::interactive::resolve_camera(&folder_name).await {
                Ok(camera_id) => {
                    match client::create_camera_instance(&camera_id, event_id, &folder_name).await {
                        Ok(_) => println!("[OK]"),
                        Err(e) => println!("[ERROR] {}", e),
                    }
                }
                Err(e) => println!("[ERROR] {}", e),
            }
        }
    }    
}
