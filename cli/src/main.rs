mod client;
mod interactive;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "continuum")]
#[command(about = "Free Video Continuum CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Зарегистрировать диск или папку
    Add {
        /// Путь к диску или папке (если не указан — интерактивный режим)
        path: Option<String>,
    },
    /// Управление дисками
    #[command(subcommand)]
    Disk(DiskCommands),
    /// Список дисков (алиас для disk ls)
    Ls,
    /// Запустить каталогизацию
    Catalog,
    /// Показать статус
    Status,
    /// Управление медиа-папками (roots)
    #[command(subcommand)]
    Roots(RootsCommands),
}

#[derive(Subcommand)]
enum DiskCommands {
    /// Список дисков
    Ls,
    /// Проверить доступность дисков
    Check,
}

#[derive(Subcommand)]
enum RootsCommands {
    /// Добавить медиа-папку к диску
    Add {
        /// Подстрока для поиска диска (по ID или label)
        contains: Option<String>,
        /// Относительный путь к папке (если не указан - интерактивный режим)
        path: Option<String>,
    },
}

fn print_disks(disks: &[client::DiskInfo]) {
    if disks.is_empty() {
        println!("Нет зарегистрированных дисков");
    } else {
        println!("{:<36} {:<20} {:<30} {:<10} {:<10}", "ID", "LABEL", "PATH", "TYPE", "AVAILABLE");
        println!("{}", "-".repeat(106));
        for d in disks {
            println!("{:<36} {:<20} {:<30} {:<10} {:<10}",
                d.disk_id,
                d.label,
                d.mount_path,
                d.disk_type,
                if d.is_available { "[OK]" } else { "[--]" }
            );
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { path } => {
            match path {
                Some(p) => {
                    println!("Добавление диска: {}", p);
                    match client::add_disk(&p, &p, "fixed").await {
                        Ok(resp) => println!("[OK] Диск зарегистрирован: {} ({})", resp.disk_id, resp.message),
                        Err(e) => eprintln!("[ERROR] {}", e),
                    }
                }
                None => {
                    interactive::add_disk_wizard().await;
                }
            }
        }
        Commands::Disk(cmd) => match cmd {
            DiskCommands::Ls => {
                match client::list_disks().await {
                    Ok(disks) => print_disks(&disks),
                    Err(e) => eprintln!("[ERROR] {}", e),
                }
            }
            DiskCommands::Check => {
                match client::check_disks().await {
                    Ok(msg) => println!("[OK] {}", msg),
                    Err(e) => eprintln!("[ERROR] {}", e),
                }
            }
        },
        Commands::Ls => {
            match client::list_disks().await {
                Ok(disks) => print_disks(&disks),
                Err(e) => eprintln!("[ERROR] {}", e),
            }
        }
        Commands::Catalog => {
            println!("Режим каталогизации — в разработке");
        }
        Commands::Status => {
            println!("Статус — в разработке");
        }
        Commands::Roots(cmd) => match cmd {
            RootsCommands::Add { contains, path } => {
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
                        println!("Интерактивный режим roots add — в разработке");
                    }
                }
            }
        },
    }
}