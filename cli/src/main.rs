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
}

#[derive(Subcommand)]
enum DiskCommands {
    /// Список дисков
    Ls,
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
                    match client::add_disk(&p, &p, "physical").await {
                        Ok(resp) => println!("[OK] Диск зарегистрирован: {} ({})", resp.disk_id, resp.message),
                        Err(e) => eprintln!("[ERROR] {}", e),
                    }
                }
                None => {
                    println!("Интерактивный мастер — в разработке");
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
    }
}