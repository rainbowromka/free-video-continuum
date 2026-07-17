

use actix_web::{web, App, HttpServer, HttpResponse};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_actix_web::TracingLogger;

use db::Database;
use std::sync::Mutex;

mod db;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Инициализация логирования
    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Запуск Free Video Continuum Server...");

    let db = Database::open("continuum.db")
        .expect("Не удалось открыть базу данных");
    info!("База данных открыта");    

    let conn = web::Data::new(Mutex::new(db.into_connection()));

    info!("Сервер запущен на http://127.0.0.1:9090");

    HttpServer::new(move || {
        App::new()
            .app_data(conn.clone())
            .wrap(TracingLogger::default())
            .route("/health", web::get().to(health_check))
            .route("/api/admin/disks", web::post().to(api::disks::add_disk))
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