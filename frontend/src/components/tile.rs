use topcoat::{
    Result,
    view::{StaticClass, View, class, component, view},
};

/// A tile's colour pair.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TileTone {
    Light,
    #[default]
    Brand,
    Mint,
    Plum,
}

impl TileTone {
    fn classes(self) -> StaticClass {
        match self {
            Self::Light => class!("bg-white text-plum-950"),
            Self::Brand => class!("bg-brand-950 text-brand-400"),
            Self::Mint => class!("bg-mint-950 text-mint-400"),
            Self::Plum => class!("bg-plum-900 text-plum-200"),
        }
    }
}

/// The size of a [`tile`]: 22, 26, 32 or 40px.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TileSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
}

impl TileSize {
    fn classes(self) -> StaticClass {
        match self {
            Self::Xs => class!("size-[22px] text-[11px]"),
            Self::Sm => class!("size-[26px] text-[12px]"),
            Self::Md => class!("size-[32px] text-[15px]"),
            Self::Lg => class!("size-[40px] text-[19px]"),
        }
    }
}

/// Apple's app-icon corner.
pub const SQUIRCLE: StaticClass = class!(
    "rounded-[22.6%] supports-[corner-shape:squircle]:rounded-[41%] [corner-shape:squircle]"
);

/// A project's identity mark: its initials on its own colour.
#[component]
pub async fn tile(
    initials: &str,
    #[default] tone: TileTone,
    #[default] size: TileSize,
    #[default] label: &str,
    #[default] decorative: bool,
) -> Result<impl View> {
    Ok(view! {
        <span
            role=((!decorative).then_some("img"))
            aria-label=((!decorative).then_some(label))
            aria-hidden=(decorative.then_some("true"))
            class=(class!(
                "inline-flex shrink-0 items-center justify-center leading-none font-bold \
                 select-none",
                SQUIRCLE,
                size.classes(),
                tone.classes(),
            ))
        >
            (initials)
        </span>
    })
}
