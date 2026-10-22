use topcoat::{
    Result,
    view::{Attributes, StaticClass, View, class, component, view},
};

use super::tone::Color;

const INPUT: StaticClass = class!(
    "h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base \
     shadow-xs transition-[color,box-shadow] outline-none \
     selection:bg-primary selection:text-primary-foreground \
     file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm \
     file:font-medium file:text-foreground placeholder:text-muted-foreground \
     disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 \
     md:text-sm dark:bg-input/30 \
     focus-visible:border-(--tone-emphasis) focus-visible:ring-3 \
     focus-visible:ring-(--tone-emphasis)/50 \
     aria-invalid:border-error aria-invalid:ring-error/20 dark:aria-invalid:ring-error/40",
);

/// A text field.
#[component]
pub async fn input(
    #[default(Color::Primary)] color: Color,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <input
            data-slot="input"
            class=(class!(INPUT, color.tone(), attrs.remove("class")))
            (attrs)
        >
    })
}
