use topcoat::view::{StaticClass, class};

/// One of the seven colours a component's `color` prop can name.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Color {
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Error,
    #[default]
    Neutral,
}

impl Color {
    /// The `tone-*` utility for this colour.
    pub fn tone(self) -> StaticClass {
        match self {
            Self::Primary => class!("tone-primary"),
            Self::Secondary => class!("tone-secondary"),
            Self::Success => class!("tone-success"),
            Self::Info => class!("tone-info"),
            Self::Warning => class!("tone-warning"),
            Self::Error => class!("tone-error"),
            Self::Neutral => class!("tone-neutral"),
        }
    }
}
