//! Documents of blocks in MongoDB, as stored and as the site renders them.

use serde::{Deserialize, Serialize};

use super::{content::MediaRef, diagram::Diagram};

/// One block of a document as stored in MongoDB. Images name their file in `media`
/// (`<sha256>.<ext>`), so a document means the same on any database holding the files.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StoredBlock {
    Text {
        text: String,
    },
    Heading {
        text: String,
    },
    Image {
        file: String,
        #[serde(default)]
        caption: String,
    },
    Gallery {
        files: Vec<String>,
    },
    Code {
        #[serde(default)]
        lang: String,
        code: String,
    },
    Quote {
        text: String,
        #[serde(default)]
        author: String,
    },
    List {
        items: Vec<String>,
    },
    /// A numbered sequence, such as a request traced step by step.
    Steps {
        items: Vec<String>,
    },
    Diagram(Diagram),
    Faq {
        items: Vec<FaqItem>,
    },
}

/// A question and its answer, opened in place.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FaqItem {
    pub question: String,
    pub answer: String,
}

/// A document found by one field, unique in its collection.
pub trait Keyed {
    /// The field.
    const KEY: &'static str;
    fn key(&self) -> &str;
}

/// A project's case study: its page below the project's record in Postgres.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CaseStudyDocument {
    /// The slug of the project in Postgres it belongs to.
    pub project: String,
    pub blocks: Vec<StoredBlock>,
}

impl Keyed for CaseStudyDocument {
    const KEY: &'static str = "project";
    fn key(&self) -> &str {
        &self.project
    }
}

/// A page made of blocks, such as `about` or `contact`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageDocument {
    pub slug: String,
    /// The file of the page's picture in `media`, such as the portrait on `about`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    pub blocks: Vec<StoredBlock>,
}

impl Keyed for PageDocument {
    const KEY: &'static str = "slug";
    fn key(&self) -> &str {
        &self.slug
    }
}

impl StoredBlock {
    /// The media files the block shows.
    pub fn files(&self) -> &[String] {
        match self {
            Self::Image { file, .. } => std::slice::from_ref(file),
            Self::Gallery { files } => files,
            _ => &[],
        }
    }
}

/// A page as the site renders it: its picture, if any, and its blocks.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Page {
    pub image: Option<MediaRef>,
    pub blocks: Vec<Block>,
}

/// A block as the site renders it, with media resolved to URLs. Blocks pointing at
/// missing media are dropped.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Text { text: String },
    Heading { text: String },
    Image { media: MediaRef, caption: String },
    Gallery { media: Vec<MediaRef> },
    Code { lang: String, code: String },
    Quote { text: String, author: String },
    List { items: Vec<String> },
    Steps { items: Vec<String> },
    Diagram(Diagram),
    Faq { items: Vec<FaqItem> },
}
