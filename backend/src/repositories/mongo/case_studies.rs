//! Case studies, keyed by their project's slug.

use mongodb::Database;

use super::documents;
use crate::models::CaseStudyDocument;

pub const COLLECTION: &str = "case_studies";
/// The field naming the project.
pub const KEY: &str = "project";

pub async fn find(
    db: &Database,
    project: &str,
) -> mongodb::error::Result<Option<CaseStudyDocument>> {
    documents::find(db, COLLECTION, project).await
}

pub async fn all(db: &Database) -> mongodb::error::Result<Vec<CaseStudyDocument>> {
    documents::all(db, COLLECTION).await
}
