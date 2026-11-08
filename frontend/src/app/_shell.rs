//! The frame around every page: the navigation bar and the footer. A group module
//! (its name starts with `_`), so it adds no URL; it lives under `app` because it links
//! to the routes.

use rust_i18n::t;
use topcoat::{
    Result,
    context::Cx,
    router::href,
    view::{View, component, view},
};

use crate::{
    components::{
        navigation::{
            brand_link::brand_link,
            footer::footer,
            footer_column::{footer_column, footer_link, footer_note},
            nav_bar::nav_bar,
            nav_link::{NavEmphasis, nav_link},
        },
        typography::rich_text::rich_text,
    },
    composables::{route, site::site},
};

/// The site's navigation: a bar of dark glass across the full width of the top of every
/// page, its contents in line with the page's, readable over the hero's photos and over
/// the page, marking the section the visitor is in.
#[component]
pub async fn site_nav(cx: &Cx) -> Result<impl View> {
    let current = |path: String| route::is_current(cx, &path);
    let home_current = route::is_home(cx);
    let projects_current = current(href!(super::projects::page).resolve(cx));
    let about_current = current(href!(super::about::page).resolve(cx));
    let contact_current = current(href!(super::_marketing::contact::page).resolve(cx));
    let name = site(cx).await.name;
    let (label, home) = (t!("nav.label"), t!("nav.home"));

    Ok(view! {
        nav_bar(
            label: &label,
            brand: view! { brand_link(href: href!(super::home), label: &home, name: &name) }.into(),
            action: view! {
                nav_link(
                    href: href!(super::_marketing::contact::page),
                    emphasis: NavEmphasis::Pill,
                    current: contact_current,
                    (t!("nav.contact"))
                )
            }.into(),
            nav_link(href: href!(super::home), current: home_current, (t!("nav.home_link")))
            nav_link(href: href!(super::projects::page), current: projects_current, (t!("nav.projects")))
            nav_link(href: href!(super::about::page), current: about_current, (t!("nav.about")))
        )
    })
}

/// The footer: columns of links under small headings, as Adobe lays them out, then a
/// slim row with the copyright.
#[component]
pub async fn site_footer(cx: &Cx) -> Result<impl View> {
    let site = site(cx).await;
    let top = t!("footer.top");
    let (explore, elsewhere, this_site, built_with_title) = (
        t!("footer.explore"),
        t!("footer.elsewhere"),
        t!("footer.this_site"),
        t!("footer.built_with"),
    );

    Ok(view! {
        footer(
            note: &site.footer,
            top: &top,
            footer_column(title: &explore, nav: true,
                footer_link(href: href!(super::home), (t!("footer.home")))
                footer_link(href: href!(super::projects::page), (t!("nav.projects")))
                footer_link(href: href!(super::about::page), (t!("about.title")))
                footer_link(href: href!(super::_marketing::contact::page), (t!("nav.contact")))
            )
            footer_column(title: &elsewhere, nav: true,
                footer_link(href: site.github_url, external: true, (t!("footer.github")))
                footer_link(href: site.linkedin_url, external: true, (t!("footer.linkedin")))
            )
            footer_column(title: &this_site, wide: true,
                footer_link(href: site.source_url, external: true, (t!("footer.source")))
                footer_note((site.stack))
            )
            footer_column(title: &built_with_title, wide: true,
                footer_note(rich_text(text: &site.built_with))
            )
        )
    })
}
