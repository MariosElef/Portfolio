mod app;
mod components;
mod composables;
mod features;
mod middleware;
mod services;
mod types;
mod utils;

rust_i18n::i18n!("locales", fallback = "en");

#[tokio::main]
async fn main() {
    topcoat::start(app::router()).await.unwrap();
}
