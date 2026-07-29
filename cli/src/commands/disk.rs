use crate::client;
use crate::display;

pub async fn handle_ls() {
    let active = client::get_active_state().await.ok();
    let active_disk_id = active.as_ref().and_then(|s| s.disk_id.as_deref());
    match client::list_disks().await {
        Ok(disks) => display::print_disks(&disks, active_disk_id),
        Err(e) => eprintln!("[ERROR] {}", e),
    }
}

pub async fn handle_check() {
    match client::check_disks().await {
        Ok(msg) => println!("[OK] {}", msg),
        Err(e) => eprintln!("[ERROR] {}", e),
    }
}

pub async fn handle_use(contains: &str) {
    match client::search_disks(contains).await {
        Ok(disks) if disks.len() == 1 => {
            let disk = &disks[0];
            match client::set_active_disk(&disk.disk_id).await {
                Ok(_) => println!("[OK] Активный диск: {} ({})", disk.label, disk.disk_id),
                Err(e) => eprintln!("[ERROR] {}", e),
            }
        }
        Ok(disks) if disks.is_empty() => eprintln!("[ERROR] Диск не найден"),
        Ok(disks) => {
            println!("Найдено несколько дисков, уточните:");
            for d in &disks {
                println!("  {} - {}", d.disk_id, d.label);
            }
        }
        Err(e) => eprintln!("[ERROR] {}", e),
    }
}

pub async fn handle_add(path: Option<String>) {
    match path {
        Some(p) => {
            println!("Добавление диска: {}", p);
            match client::add_disk(&p, &p, "fixed").await {
                Ok(resp) => println!("[OK] Диск зарегистрирован: {} ({})", resp.disk_id, resp.message),
                Err(e) => eprintln!("[ERROR] {}", e),
            }
        }
        None => {
            crate::interactive::add_disk_wizard().await;
        }
    }
}

pub async fn handle_status() {
    match client::get_active_state().await {
        Ok(state) => {
            if let Some(ref disk_id) = state.disk_id {
                match client::search_disks(disk_id).await {
                    Ok(disks) if disks.len() == 1 => {
                        let disk = &disks[0];
                        println!("Активный диск:  {} ({})", disk.label, disk_id);
                    }
                    _ => println!("Активный диск:  {}", disk_id),
                }
            } else {
                println!("Активный диск:  не выбран");
            }

            if let Some(ref root_id) = state.root_id {
                if let Some(ref disk_id) = state.disk_id {
                    match client::list_roots(disk_id).await {
                        Ok(roots) => {
                            if let Some(root) = roots.iter().find(|r| &r.id == root_id) {
                                println!("Активный root:  {} ({})", root.relative_path, root_id);
                            } else {
                                println!("Активный root:  {}", root_id);
                            }
                        }
                        _ => println!("Активный root:  {}", root_id),
                    }
                }
            } else {
                println!("Активный root:  не выбран");
            }
        }
        Err(e) => eprintln!("[ERROR] {}", e),
    }
}