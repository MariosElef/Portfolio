//! Postgres: the records. Each query is a file in `sql/`, checked against the schema
//! when compiling (see `.cargo/config.toml`).

pub mod media;
pub mod messages;
pub mod profile;
pub mod projects;
pub mod records;
