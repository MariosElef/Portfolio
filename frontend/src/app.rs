use rust_i18n::t;
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    font::RouterBuilderFontExt,
    icon::iconify::iconify_icon,
    router::{
        Body, Next, Router, Slot, href, layer, layout, module_router, page, response::Response,
    },
    runtime::RouterBuilderRuntimeExt,
    view::{View, component, suspense, view},
};

use crate::{
    components::{
        actions::icon_link::icon_link,
        layout::{
            document::{INTER, VARELA_ROUND, document},
            frame::frame,
            section::{SectionSpacing, section},
            sheet::sheet,
        },
        loading::{
            cards_skeleton::cards_skeleton, hero_skeleton::hero_skeleton,
            section_skeleton::section_skeleton,
        },
    },
    composables::{projects::featured_projects, route, site::site as site_copy},
    features::{hero::home_hero, projects::grid::project_grid},
    middleware::{
        client_ip::with_client_ip, security::security_headers, streaming::with_suspense_mode,
    },
    services::client::ApiClient,
};

mod _marketing;
mod _shell;
mod about;
mod projects;

/// Builds the application's router.
pub fn router() -> Router {
    module_router!()
        .font(VARELA_ROUND)
        .font(INTER)
        .assets(AssetBundle::load().unwrap())
        .app_context(ApiClient::from_env().expect("API_URL must be a base URL"))
        .runtime()
        .build()
}

/// Around every request: the visitor's address made available, whether the page streams
/// in (see `with_suspense_mode`), then the security headers added to the response. One
/// layer, as a path takes only one.
#[layer]
async fn site(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let cx = with_suspense_mode(&with_client_ip(cx));
    security_headers(&cx, body, next).await
}

#[layout]
async fn root(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let copy = site_copy(cx).await;

    Ok(view! {
        document(
            title: &copy.title,
            frame(
                // The home page runs beneath the bar; the others start below it.
                offset: !route::is_home(cx),
                bar: view! { _shell::site_nav() }.into(),
                footer: view! { _shell::site_footer() }.into(),
                (slot)
            )
        )
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        suspense(fallback: view! { hero_skeleton() }, home_hero())
        sheet(
            suspense(
                fallback: view! { section_skeleton(cards_skeleton(count: 3)) },
                featured()
            )
        )
    })
}

/// The featured projects, with the way to all of them and to GitHub.
#[component]
async fn featured(cx: &Cx) -> Result<impl View> {
    let featured = projects::entries(cx, featured_projects(cx).await?);
    let more = href!(projects::page).resolve(cx);
    let title = t!("home.featured");
    let github = site_copy(cx).await.github_url;

    Ok(view! {
        section(
            title: &title,
            spacing: SectionSpacing::Around,
            action: view! {
                icon_link(href: github, icon_data: iconify_icon!("lucide:github"), external: true, (t!("home.github")))
            }.into(),
            project_grid(entries: featured, lead: true, more: Some(more))
        )
    })
}
