use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    font::{Font, RouterBuilderFontExt, fontsource::fontsource_font},
    icon::{icon, iconify::iconify_icon},
    router::{Router, Slot, layout, module_router, page},
    runtime::RouterBuilderRuntimeExt,
    tailwind,
    view::{View, attributes, view},
};

use crate::components::{
    avatar::{avatar, avatar_fallback},
    banner::banner,
    button::{ButtonShape, ButtonSize, ButtonVariant, button, button_variants},
    input::input,
    label::label,
    project_card::project_card,
    tile::TileTone,
    toggle_group::{toggle, toggle_group},
    tone::Color,
};

/// The display face. It ships weight 400 only; heavier weights are synthesised.
const VARELA_ROUND: Font = fontsource_font!(VARELA_ROUND, weight: 400, style: Normal);

/// Underneath Varela Round, for the glyphs it lacks, such as Greek.
const INTER: Font =
    fontsource_font!(INTER, weight: [400, 700], style: Normal, subset: [Latin, Greek]);

/// Builds the application's router.
pub fn router() -> Router {
    module_router!()
        .font(VARELA_ROUND)
        .font(INTER)
        .assets(AssetBundle::load().unwrap())
        .runtime()
        .build()
}

#[layout]
async fn root(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Marios Elef"</title>
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::font::link(font: VARELA_ROUND)
                topcoat::font::link(font: INTER)
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body>(slot)</body>
        </html>
    })
}

#[page]
async fn home() -> Result<impl View> {
    let link = |color, variant, size| button_variants(color, ButtonShape::Pill, variant, size);

    Ok(view! {
        banner(
            color: Color::Info,
            action: view! {
                <a href="#contact" class=(link(Color::Neutral, ButtonVariant::Outline, ButtonSize::Lg))>
                    "Get in touch"
                </a>
            }.into(),
            "Open to new work."
        )

        <main class="mx-auto flex max-w-5xl flex-col gap-24 px-4 py-20 sm:px-6">
            <section class="flex flex-col items-start gap-6">
                avatar(
                    color: Color::Primary,
                    attrs: attributes! { class="size-20 text-2xl" },
                    avatar_fallback("ME")
                )
                <h1 class="text-4xl font-bold tracking-tight sm:text-5xl">"Marios Elef"</h1>
                <p class="max-w-xl text-lg text-muted-foreground">
                    "Software engineer. Replace this line with what you build and what you care about."
                </p>
                <div class="flex flex-wrap gap-3">
                    <a href="#projects" class=(link(Color::Primary, ButtonVariant::Solid, ButtonSize::Lg))>
                        "See projects"
                    </a>
                    <a href="https://github.com/MariosElef" class=(link(Color::Neutral, ButtonVariant::Outline, ButtonSize::Lg))>
                        icon(data: iconify_icon!("lucide:github"))
                        "GitHub"
                    </a>
                </div>
            </section>

            <section id="projects" class="flex flex-col gap-8">
                <div class="flex flex-wrap items-end justify-between gap-4">
                    <h2 class="text-2xl font-bold">"Projects"</h2>
                    toggle_group(
                        attrs: attributes! { aria-label="Filter by stack" },
                        for (value, text) in [("all", "All"), ("rust", "Rust"), ("web", "Web")] {
                            toggle(
                                attrs: attributes! { name="stack" value=(value) checked=(value == "all") },
                                (text)
                            )
                        }
                    )
                </div>
                <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
                    project_card(
                        initials: "St",
                        tone: TileTone::Mint,
                        name: "stackup",
                        description: "The command line this portfolio was scaffolded and is run with.",
                        tags: vec!["Rust", "Docker", "CLI"],
                        links: view! {
                            <a
                                href="https://github.com/MariosElef/stackup"
                                aria-label="stackup on GitHub"
                                class=(link(Color::Neutral, ButtonVariant::Ghost, ButtonSize::Icon))
                            >
                                icon(data: iconify_icon!("lucide:github"))
                            </a>
                        }.into(),
                        action: view! {
                            <a href="https://github.com/MariosElef/stackup" class=(link(Color::Neutral, ButtonVariant::Outline, ButtonSize::Md))>
                                "View"
                            </a>
                        }.into(),
                    )
                    project_card(
                        initials: "Pr",
                        tone: TileTone::Brand,
                        name: "Project two",
                        description: "A placeholder: describe the problem it solved in one line.",
                        tags: vec!["Vue", "Nuxt"],
                        action: view! {
                            <a href="#" class=(link(Color::Neutral, ButtonVariant::Outline, ButtonSize::Md))>"View"</a>
                        }.into(),
                    )
                    project_card(loading: true)
                </div>
            </section>

            <section id="contact" class="flex flex-col gap-6">
                <h2 class="text-2xl font-bold">"Contact"</h2>
                <form class="flex max-w-md flex-col gap-4" method="post" action="#">
                    <div class="flex flex-col gap-2">
                        label(attrs: attributes! { for="email" }, "Email")
                        input(attrs: attributes! { id="email" name="email" type="email" placeholder="you@example.com" required="" })
                    </div>
                    <div class="flex flex-col gap-2">
                        label(attrs: attributes! { for="message" }, "Message")
                        input(attrs: attributes! { id="message" name="message" placeholder="Say hello" required="" })
                    </div>
                    button(
                        color: Color::Primary,
                        variant: ButtonVariant::Solid,
                        size: ButtonSize::Lg,
                        attrs: attributes! { type="submit" class="self-start" },
                        "Send"
                    )
                </form>
            </section>
        </main>
    })
}
