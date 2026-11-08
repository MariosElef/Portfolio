//! The data the API works with, by domain. Everything is re-exported here, so callers
//! name a type as `crate::models::Project` wherever it lives.

mod content;
mod diagram;
mod documents;
mod github;
mod home;
mod media;
mod messages;
mod records;
mod site;

pub use content::*;
pub use diagram::*;
pub use documents::*;
pub use github::*;
pub use home::*;
pub use media::*;
pub use messages::*;
pub use records::*;
pub use site::*;
