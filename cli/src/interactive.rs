use dialoguer::{Confirm, Input, Select};
use std::env;

pub async fn add_disk_wizard() {
    println!("=== Мастер добавления диска ===\n");

    // Шаг 1: определяем путь
    let current_dir = env::current_dir().unwrap_or_default();
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