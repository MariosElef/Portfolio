use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::tone::Color;

/// A full-width message bar: an announcement, an availability notice, maintenance.
#[component]
pub async fn banner(
    #[default(Color::Info)] color: Color,
    #[default] action: Child<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let role = if color == Color::Error {
        "alert"
    } else {
        "status"
    };
    let symbol = if color == Color::Info {
        iconify_icon!("lucide:info")
    } else {
        iconify_icon!("lucide:triangle-alert")
    };

    Ok(view! {
        <div
            role=(role)
            class=(class!(
                "flex min-h-14 items-center gap-3 bg-(--tone) px-6 py-2 text-(--tone-foreground)",
                color.tone(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            icon(data: symbol, attrs: attributes! { class="size-5 shrink-0" })
            <p class="flex-1 text-[15px] font-medium">(child)</p>
            (action)
        </div>
    })
}
