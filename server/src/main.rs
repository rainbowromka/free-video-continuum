use actix_web::{web, App, HttpServer, HttpResponse};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Инициализация логирования
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Запуск Free Video Continuum Server...");

    info!("Сервер запущен на http://127.0.0.1:9090");

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health", web::get().to(health_check))
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