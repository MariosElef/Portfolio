mod app;
mod components;
mod features;

rust_i18n::i18n!("locales", fallback = "en");

#[tokio::main]
async fn main() {
    topcoat::start(app::router()).await.unwrap();
}
