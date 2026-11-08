//! The content the site starts with, written on start wherever it is missing: the
//! records into Postgres, the documents into MongoDB. Edits made in the databases
//! afterwards stay. Migrations hold schema only.

pub mod blocks;
pub mod case_studies;
pub mod home;
pub mod legend;
pub mod pages;
pub mod records;
pub mod site;
