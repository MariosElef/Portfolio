use topcoat::{
    Result,
    runtime::link,
    view::{AttributeValueViewParts, Child, View, attributes, component, view},
};

use crate::components::actions::external_link::external_link;

const LINK: &str = "text-sm text-muted-foreground transition-colors hover:text-foreground";

/// A column's heading over its list.
#[component]
async fn column_body(title: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <h2 class="mb-4 text-sm font-bold text-foreground">(title)</h2>
        <ul class="flex flex-col gap-3">(child)</ul>
    })
}

/// A column of the footer: a small heading over a list of [`footer_link`]s and
/// [`footer_note`]s. With `nav`, the column is a landmark named by its heading. With
/// `wide`, it takes the footer's full width on a phone, for longer text.
#[component]
pub async fn footer_column(
    title: &str,
    #[default] nav: bool,
    #[default] wide: bool,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let span = if wide { "col-span-2 sm:col-span-1" } else { "" };
    Ok(view! {
        if nav {
            <nav aria-label=(title) class=(span)>column_body(title: title, (child))</nav>
        } else {
            <div class=(span)>column_body(title: title, (child))</div>
        }
    })
}

/// A link in a [`footer_column`]: a page of this site, or with `external` another site,
/// in a new tab.
#[component]
pub async fn footer_link<H>(
    href: H,
    #[default] external: bool,
    #[default] child: Child<'_>,
) -> Result<impl View>
where
    H: AttributeValueViewParts + Send,
{
    Ok(view! {
        <li>
            if external {
                external_link(href: href, attrs: attributes! { class=(LINK) }, (child))
            } else {
                link(href: href, attrs: attributes! { class=(LINK) }, (child))
            }
        </li>
    })
}

/// Plain text in a [`footer_column`], beside its links.
#[component]
pub async fn footer_note(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <li class="text-sm leading-relaxed text-muted-foreground">(child)</li>
    })
}
