use actix_web::{web, App, HttpServer, HttpResponse};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_actix_web::TracingLogger;

use db::Database;
use std::sync::Mutex;

mod db;
mod api;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    #[cfg(windows)]
    {
        enable_ansi_support::enable_ansi_support().ok();
    }

    // Инициализация логирования
    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Запуск Free Video Continuum Server...");

    let mut db = Database::open("continuum.db")
        .expect("Не удалось открыть базу данных");
    info!("База данных открыта");

    // Синхронизация дисков при старте
    storage::disks::sync_disks(db.conn_ref())
        .unwrap_or_else(|e| info!("Предупреждение при синхронизации дисков: {}", e));

    let conn = web::Data::new(Mutex::new(db.into_connection()));

    // Фоновая задача: проверка дисков каждые 30 секунд
    let conn_clone = conn.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            let c = conn_clone.lock().unwrap();
            if let Err(e) = storage::disks::sync_disks(&c) {
                info!("Ошибка фоновой синхронизации дисков: {}", e);
            }
        }
    });

    info!("Сервер запущен на http://127.0.0.1:9090");

    HttpServer::new(move || {
        App::new()
            .app_data(conn.clone())
            .wrap(TracingLogger::default())
            .route("/health", web::get().to(health_check))
            .route("/api/admin/disks", web::post().to(api::disks::add_disk))
            .route("/api/admin/disks", web::get().to(api::disks::list_disks))
            .route("/api/admin/disks/check", web::post().to(api::disks::check_disks))
            .route("/api/admin/disks/{disk_id}/media-roots", web::post().to(api::disks::add_media_root))
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "Free Video Continuum Server"
    }))
}