use topcoat::{
    Result,
    icon::{IconData, icon},
    view::{Attributes, StaticClass, View, attributes, class, component, view},
};

/// What every text field shares: its border and background, its placeholder, and how it
/// looks focused, invalid or disabled. Both fields here use it.
const FIELD: StaticClass = class!(
    "w-full rounded-lg border border-white/10 bg-black/40 text-foreground \
     transition-[border-color,box-shadow] outline-none \
     selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground/70 \
     hover:border-white/20 \
     disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 \
     focus-visible:border-white/40 focus-visible:ring-4 focus-visible:ring-white/10 \
     aria-invalid:border-error aria-invalid:ring-error/20 dark:aria-invalid:ring-error/40",
);

/// The label inside the field: as large as the text while the field is empty and not
/// focused, then small at the top once it is focused or filled. CSS alone does this,
/// from the field's `:placeholder-shown`, so the field's placeholder is a single space.
const LABEL: StaticClass = class!(
    "pointer-events-none absolute left-4 text-base text-muted-foreground \
     transition-[top,font-size,color,translate] duration-150 ease-out \
     peer-focus:text-xs peer-focus:font-medium peer-focus:text-foreground \
     peer-[:not(:placeholder-shown)]:text-xs peer-[:not(:placeholder-shown)]:font-medium",
);

/// A one-line field with its label inside it, and `icon` on the left if given, which
/// brightens while the field is focused.
#[component]
pub async fn floating_input(
    id: &str,
    label_text: &str,
    #[default] icon_data: Option<IconData>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let padded = icon_data.is_some();
    Ok(view! {
        <div class="relative">
            <input
                id=(id)
                placeholder=" "
                class=(class!(FIELD, "peer h-14 px-4 pt-5 pb-1 text-base", "pl-11" if padded, attrs.remove("class")))
                (attrs)
            >
            if let Some(data) = icon_data {
                icon(data: data, attrs: attributes! { class="pointer-events-none absolute top-1/2 left-4 size-4 -translate-y-1/2 text-muted-foreground transition-colors peer-focus:text-foreground" })
            }
            <label
                for=(id)
                class=(class!(
                    LABEL,
                    "top-1/2 -translate-y-1/2 peer-focus:top-2.5 peer-focus:translate-y-0 \
                     peer-[:not(:placeholder-shown)]:top-2.5 peer-[:not(:placeholder-shown)]:translate-y-0",
                    "left-11" if padded,
                ))
            >
                (label_text)
            </label>
        </div>
    })
}

/// A text area with its label inside it, at the top.
#[component]
pub async fn floating_textarea(
    id: &str,
    label_text: &str,
    /// What it holds to start with, such as what was typed before.
    #[default]
    value: String,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div class="relative">
            <textarea
                id=(id)
                placeholder=" "
                class=(class!(FIELD, "peer block min-h-44 resize-none px-4 pt-8 pb-3 text-base", attrs.remove("class")))
                (attrs)
            >(value)</textarea>
            <label
                for=(id)
                class=(class!(
                    LABEL,
                    "top-4 peer-focus:top-2.5 peer-[:not(:placeholder-shown)]:top-2.5",
                ))
            >
                (label_text)
            </label>
        </div>
    })
}
