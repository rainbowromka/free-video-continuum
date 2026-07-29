mod client;
mod interactive;
mod commands;
mod display;

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
    /// Установить активный диск
    Use {
        /// Подстрока для поиска диска
        contains: String,
    },
    /// Зарегистрировать диск или папку
    Add {
        /// Путь к диску или папке (если не указан — интерактивный режим)
        path: Option<String>,
    },
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
    /// Список медиа-папок диска
    Ls {
        /// Подстрока для поиска диска (если не указан — активный диск)
        contains: Option<String>,
    },
    /// Установить активный root
    Use {
        /// Подстрока для поиска root (по relative_path)
        contains: String,
    },    
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Add { path } => commands::disk::handle_add(path).await,
        Commands::Disk(cmd) => match cmd {            
            DiskCommands::Ls => commands::disk::handle_ls().await,
            DiskCommands::Check => commands::disk::handle_check().await,
            DiskCommands::Use { contains } => commands::disk::handle_use(&contains).await,
            DiskCommands::Add { path } => commands::disk::handle_add(path).await,
        },
        Commands::Ls => commands::disk::handle_ls().await,
        Commands::Catalog => {
            println!("Режим каталогизации — в разработке");
        }
        Commands::Status => commands::disk::handle_status().await,        
        Commands::Roots(cmd) => match cmd {
            RootsCommands::Add { contains, path } => commands::roots::handle_add(contains, path).await,
            RootsCommands::Ls { contains } => commands::roots::handle_ls(contains).await,
            RootsCommands::Use { contains } => commands::roots::handle_use(&contains).await,
        },
    }
}