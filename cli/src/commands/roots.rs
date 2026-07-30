use crate::client;
use crate::display;

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
    match client::scan_events().await {
        Ok(msg) => println!("[OK] {}", msg),
        Err(e) => eprintln!("[ERROR] {}", e),
    }
}