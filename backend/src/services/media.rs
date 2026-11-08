use std::path::Path;

use axum::body::Body;
use futures_util::TryStreamExt;
use sha2::{Digest, Sha256};
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{models::Uploaded, repositories::postgres::media, state::AppState};

pub const MAX_UPLOAD_BYTES: u64 = 200 * 1024 * 1024;

#[derive(Debug)]
pub enum UploadError {
    UnsupportedType,
    Empty,
    TooLarge,
    Failed(anyhow::Error),
}

impl<E: Into<anyhow::Error>> From<E> for UploadError {
    fn from(error: E) -> Self {
        Self::Failed(error.into())
    }
}

fn extension(content_type: &str) -> Option<&'static str> {
    Some(match content_type {
        "video/mp4" => "mp4",
        "video/webm" => "webm",
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/avif" => "avif",
        _ => return None,
    })
}

/// Streams `body` into the media directory under its sha256, and records it.
pub async fn upload(
    state: &AppState,
    content_type: &str,
    body: Body,
) -> Result<Uploaded, UploadError> {
    let ext = extension(content_type).ok_or(UploadError::UnsupportedType)?;
    let temp = state.media_dir.join(format!(".upload-{}", Uuid::now_v7()));

    let written = write_hashed(&temp, body).await;
    let (hash, size) = match written {
        Ok(done) => done,
        Err(error) => {
            let _ = fs::remove_file(&temp).await;
            return Err(error);
        }
    };

    let file_name = format!("{hash}.{ext}");
    let target = state.media_dir.join(&file_name);
    if fs::try_exists(&target).await? {
        fs::remove_file(&temp).await?;
    } else {
        fs::rename(&temp, &target).await?;
    }

    let size = i64::try_from(size)?;
    let id = media::insert(&state.pool, &file_name, content_type, size).await?;
    Ok(Uploaded {
        id,
        url: format!("/media/{file_name}"),
    })
}

async fn write_hashed(path: &Path, body: Body) -> Result<(String, u64), UploadError> {
    let mut file = fs::File::create(path).await?;
    let mut hasher = Sha256::new();
    let mut size = 0_u64;
    let mut stream = body.into_data_stream();

    while let Some(chunk) = stream.try_next().await? {
        size += chunk.len() as u64;
        if size > MAX_UPLOAD_BYTES {
            return Err(UploadError::TooLarge);
        }
        hasher.update(&chunk);
        file.write_all(&chunk).await?;
    }
    file.flush().await?;

    if size == 0 {
        return Err(UploadError::Empty);
    }
    Ok((hex::encode(hasher.finalize()), size))
}
