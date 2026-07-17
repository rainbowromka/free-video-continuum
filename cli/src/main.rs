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
    /// Запустить каталогизацию
    Catalog,
    /// Показать статус
    Status,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { path } => {
            match path {
                Some(p) => add_disk_quick(&p).await,
                None => add_disk_interactive().await,
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

async fn add_disk_quick(path: &str) {
    println!("Добавляю диск: {}", path);
    // TODO: HTTP-запрос к серверу
}

async fn add_disk_interactive() {
    println!("Интерактивный мастер добавления диска — в разработке");
    // TODO: dialoguer
}