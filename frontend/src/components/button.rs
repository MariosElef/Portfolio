use topcoat::{
    Result,
    view::{Attributes, Child, Class, StaticClass, View, class, component, view},
};

use super::tone::Color;

/// The corner of a [`button`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ButtonShape {
    #[default]
    Pill,
    Squircle,
}

impl ButtonShape {
    fn classes(self) -> StaticClass {
        match self {
            Self::Pill => class!("rounded-full"),
            Self::Squircle => class!(
                "rounded-lg [corner-shape:squircle] supports-[corner-shape:squircle]:rounded-2xl"
            ),
        }
    }
}

/// How a [`button`]'s colour is applied.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ButtonVariant {
    Solid,
    #[default]
    Outline,
    Soft,
    Subtle,
    Ghost,
    Link,
}

impl ButtonVariant {
    fn classes(self) -> StaticClass {
        match self {
            Self::Solid => class!(
                "bg-(--tone) text-(--tone-foreground) hover:bg-(--tone)/90 active:bg-(--tone)/80"
            ),
            Self::Outline => class!(
                "border-2 border-(--tone-emphasis) text-(--tone-emphasis) \
                 hover:bg-(--tone-emphasis)/10 active:bg-(--tone-emphasis)/20"
            ),
            Self::Soft => class!(
                "bg-(--tone)/10 text-(--tone-emphasis) hover:bg-(--tone)/15 active:bg-(--tone)/20"
            ),
            Self::Subtle => class!(
                "border border-(--tone-emphasis)/25 bg-(--tone)/10 text-(--tone-emphasis) \
                 hover:bg-(--tone)/15 active:bg-(--tone)/20"
            ),
            Self::Ghost => {
                class!("text-(--tone-emphasis) hover:bg-(--tone)/10 active:bg-(--tone)/15")
            }
            Self::Link => class!("text-(--tone-emphasis) underline-offset-4 hover:underline"),
        }
    }
}

/// The height and padding of a [`button`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ButtonSize {
    Xs,
    #[default]
    Md,
    Lg,
    Xl,
    Icon,
    IconXs,
    IconLg,
}

impl ButtonSize {
    fn classes(self) -> StaticClass {
        match self {
            Self::Xs => class!(
                "h-6 gap-1 px-2 text-xs has-[>svg]:px-1.5 [&_svg:not([class*='size-'])]:size-3"
            ),
            Self::Md => class!("h-7 px-3.5 text-[13px] has-[>svg]:px-3"),
            Self::Lg => class!("h-9 px-4 text-sm has-[>svg]:px-3"),
            Self::Xl => class!("h-10 px-6 text-sm has-[>svg]:px-4"),
            Self::Icon => class!("size-7"),
            Self::IconXs => class!("size-6 [&_svg:not([class*='size-'])]:size-3"),
            Self::IconLg => class!("size-9"),
        }
    }
}

const BASE: StaticClass = class!(
    "inline-flex shrink-0 cursor-pointer items-center justify-center gap-2 leading-none \
     font-bold whitespace-nowrap transition-all outline-none select-none \
     focus-visible:ring-3 focus-visible:ring-ring/50 \
     disabled:pointer-events-none disabled:opacity-50 \
     aria-invalid:ring-error/20 dark:aria-invalid:ring-error/40 \
     [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
);

/// Button classes for an element that is not a `<button>`, such as a link.
#[must_use]
pub fn button_variants(
    color: Color,
    shape: ButtonShape,
    variant: ButtonVariant,
    size: ButtonSize,
) -> Class<(
    StaticClass,
    StaticClass,
    StaticClass,
    StaticClass,
    StaticClass,
)> {
    class!(
        BASE,
        color.tone(),
        shape.classes(),
        variant.classes(),
        size.classes()
    )
}

/// A button in one of seven colours, two shapes and six variants.
#[component]
pub async fn button(
    #[default] color: Color,
    #[default] shape: ButtonShape,
    #[default] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            data-slot="button"
            class=(class!(button_variants(color, shape, variant, size), attrs.remove("class")))
            (attrs)
        >
            (child)
        </button>
    })
}
