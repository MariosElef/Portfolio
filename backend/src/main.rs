use tracing_subscriber::EnvFilter;

mod checks;
mod clients;
mod config;
mod jobs;
mod middleware;
mod models;
mod repositories;
mod routes;
mod seeds;
mod services;
mod startup;
mod state;
mod upgrades;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let config = config::Config::from_env()?;
    let addr = config.bind_addr.clone();
    let state = startup::state(config).await?;

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "listening");

    // With each connection's address, the rate limit's last resort for telling visitors apart.
    let app = routes::router(state).into_make_service_with_connect_info::<std::net::SocketAddr>();
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
