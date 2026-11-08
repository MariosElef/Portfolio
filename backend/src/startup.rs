//! Everything between reading the settings and serving: the databases connected and
//! brought up to date, the starting content written, and the background jobs started.

use crate::{
    checks,
    clients::{github::GitHub, mailer::Mailer},
    config::Config,
    jobs::cache_invalidation::{spawn_content_listener, spawn_document_watcher},
    models::{CaseStudyDocument, PageDocument},
    repositories::{
        mongo::{case_studies, documents, github_snapshots, home, pages, site},
        postgres::{media, records},
        redis::Redis,
    },
    seeds,
    state::{AdminToken, AppState},
    upgrades,
};

/// The state every request shares, once everything it needs is ready.
pub async fn state(config: Config) -> anyhow::Result<AppState> {
    // Postgres, brought up to date before anything is served: each file in migrations/,
    // compiled in, is applied once. Then the media folder's files are known, and the records
    // the site starts with are written where missing, their media linked by file name.
    let pool = sqlx::PgPool::connect(&config.database_url).await?;
    sqlx::migrate!().run(&pool).await?;
    tokio::fs::create_dir_all(&config.media_dir).await?;
    register_media(&pool, &config.media_dir).await?;
    seed_records(&pool).await?;

    let redis = Redis::connect(&config.redis_url).await?;
    spawn_content_listener(pool.clone(), redis.clone());

    // MongoDB: the database named in the URL, or `portfolio` when it names none.
    let client = mongodb::Client::with_uri_str(&config.mongodb_url).await?;
    let mongo = client
        .default_database()
        .unwrap_or_else(|| client.database("portfolio"));
    // Documents in an older shape are brought up to date before the seeds look for them;
    // then every link between the stores is checked.
    upgrades::run(&pool, &mongo).await?;
    seed(&mongo).await?;
    checks::run(&pool, &mongo).await?;
    spawn_document_watcher(mongo.clone(), redis.clone());

    Ok(AppState {
        pool,
        mongo,
        redis,
        // Read from the public profile, so no token is needed.
        github: GitHub::new(config.github_user),
        mailer: Mailer::new(config.smtp)?,
        media_dir: config.media_dir,
        admin_token: AdminToken(config.admin_token.into()),
        challenge_key: if config.challenge_secret.is_empty() {
            rand::random::<[u8; 32]>().to_vec().into()
        } else {
            config.challenge_secret.into_bytes().into()
        },
    })
}

/// The documents the site starts with, for any not there yet. Documents already there are
/// never touched, so edits made in the database stay.
async fn seed(mongo: &mongodb::Database) -> mongodb::error::Result<()> {
    documents::ensure_key_index::<CaseStudyDocument>(mongo, case_studies::COLLECTION).await?;
    documents::insert_missing(mongo, case_studies::COLLECTION, seeds::case_studies::all()).await?;

    // `about` goes only into an empty collection, so deleting it keeps it deleted.
    documents::ensure_key_index::<PageDocument>(mongo, pages::COLLECTION).await?;
    if documents::is_empty(mongo, pages::COLLECTION).await? {
        documents::insert_missing(mongo, pages::COLLECTION, vec![seeds::pages::about()]).await?;
    }
    documents::insert_missing(mongo, pages::COLLECTION, vec![seeds::pages::contact()]).await?;

    site::insert_if_empty(mongo, &seeds::site::site()).await?;
    home::insert_if_empty(mongo, &seeds::home::home()).await?;
    github_snapshots::ensure_index(mongo).await?;
    Ok(())
}

/// Every file in the media folder that the `media` table does not know yet, as uploads
/// would have recorded it. Files are named by their hash, so this only fills gaps.
async fn register_media(pool: &sqlx::PgPool, dir: &std::path::Path) -> anyhow::Result<()> {
    let mut entries = tokio::fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let name = entry.file_name().to_string_lossy().into_owned();
        let content_type = match name.rsplit_once('.').map(|(_, ext)| ext) {
            Some("png") => "image/png",
            Some("jpg" | "jpeg") => "image/jpeg",
            Some("webp") => "image/webp",
            Some("avif") => "image/avif",
            Some("mp4") => "video/mp4",
            Some("webm") => "video/webm",
            _ => continue,
        };
        let size = entry.metadata().await?.len() as i64;
        media::register(pool, &name, content_type, size).await?;
    }
    Ok(())
}

/// The records the site starts with, each only where it is missing.
async fn seed_records(pool: &sqlx::PgPool) -> sqlx::Result<()> {
    use seeds::records::{experience, projects, skills, tools};
    records::categories(pool, tools::CATEGORIES).await?;
    records::tools(pool, tools::TOOLS).await?;
    records::projects(pool, projects::PROJECTS).await?;
    records::roles(pool, experience::ROLES).await?;
    records::skills(pool, skills::AREAS).await?;
    Ok(())
}
