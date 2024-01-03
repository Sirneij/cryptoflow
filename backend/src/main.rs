use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let settings = backend::settings::get_settings().expect("Failed to read settings.");

    let subscriber_format = if settings.debug {
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "backend=debug,tower_http=debug,axum::rejection=debug,sqlx=debug".into()
        })
    } else {
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "backend=info,tower_http=info,axum::rejection=info,sqlx=info".into()
        })
    };

    tracing_subscriber::registry()
        .with(subscriber_format)
        .with(tracing_subscriber::fmt::layer())
        .init();

    backend::startup::Application::build(settings, None).await?;

    Ok(())
}
