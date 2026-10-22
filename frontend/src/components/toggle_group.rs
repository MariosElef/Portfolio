use topcoat::{
    Result,
    view::{Attributes, Child, PromotedStr, StaticClass, View, class, component, view},
};

use super::tone::Color;

/// How a [`toggle`] is drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToggleVariant {
    #[default]
    Chip,
    Plain,
    Outline,
}

impl ToggleVariant {
    fn classes(self) -> StaticClass {
        match self {
            Self::Chip => class!(
                "rounded-full bg-card text-card-foreground hover:bg-accent \
                 has-[:checked]:bg-(--tone) has-[:checked]:text-(--tone-foreground)"
            ),
            Self::Plain => class!(
                "rounded-md bg-transparent hover:bg-muted hover:text-muted-foreground \
                 has-[:checked]:bg-(--tone)/10 has-[:checked]:text-(--tone-emphasis)"
            ),
            Self::Outline => class!(
                "rounded-md border border-input bg-transparent shadow-xs hover:bg-accent \
                 hover:text-accent-foreground \
                 has-[:checked]:bg-(--tone)/10 has-[:checked]:text-(--tone-emphasis)"
            ),
        }
    }
}

/// The size of a [`toggle`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToggleSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ToggleSize {
    fn classes(self) -> StaticClass {
        match self {
            Self::Sm => class!("h-8 min-w-8 px-3"),
            Self::Md => class!("h-9 min-w-9 px-3.5"),
            Self::Lg => class!("h-10 min-w-10 px-4"),
        }
    }
}

/// Whether a [`toggle`] presses on its own or as one of a group.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToggleKind {
    Independent,
    #[default]
    Exclusive,
}

impl ToggleKind {
    fn input_type(self) -> PromotedStr {
        match self {
            Self::Independent => PromotedStr(&"checkbox"),
            Self::Exclusive => PromotedStr(&"radio"),
        }
    }
}

const BASE: StaticClass = class!(
    "inline-flex shrink-0 cursor-pointer items-center justify-center gap-2 text-sm \
     font-medium whitespace-nowrap transition-[color,box-shadow] select-none \
     has-[:focus-visible]:ring-3 has-[:focus-visible]:ring-ring/50 \
     has-[:disabled]:pointer-events-none has-[:disabled]:opacity-50 \
     [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
);

/// A chip that stays on when selected.
#[component]
pub async fn toggle(
    #[default] kind: ToggleKind,
    #[default] color: Color,
    #[default] variant: ToggleVariant,
    #[default] size: ToggleSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label
            data-slot="toggle"
            class=(class!(
                BASE,
                color.tone(),
                variant.classes(),
                size.classes(),
                attrs.remove("class"),
            ))
        >
            <input type=(kind.input_type()) class="sr-only" (attrs)>
            (child)
        </label>
    })
}

/// A row of [`toggle`]s. Separate pills with a gap, not a joined segmented bar.
#[component]
pub async fn toggle_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="toggle-group"
            role="group"
            class=(class!("flex w-fit flex-wrap items-center gap-3", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}
