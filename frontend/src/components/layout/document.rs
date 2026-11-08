use topcoat::{
    Result,
    font::{Font, fontsource::fontsource_font},
    tailwind,
    view::{Child, View, component, view},
};

/// The display face. It ships weight 400 only; heavier weights are synthesised.
pub const VARELA_ROUND: Font = fontsource_font!(VARELA_ROUND, weight: 400, style: Normal);

/// Underneath Varela Round, for the glyphs it lacks, such as Greek; and on its own for the
/// wordmark, the bar's links and project names.
pub const INTER: Font =
    fontsource_font!(INTER, weight: [400, 500, 700, 800], style: Normal, subset: [Latin, Greek]);

/// The whole HTML document: its head (title, styles, fonts, and Topcoat's runtime) and its
/// body. The site's motion is CSS; there are no scripts of its own.
#[component]
pub async fn document(title: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="en" class="dark scroll-pt-24">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>(title)</title>
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::font::link(font: VARELA_ROUND)
                topcoat::font::link(font: INTER)
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body class="flex min-h-dvh flex-col overflow-x-clip">(child)</body>
        </html>
    })
}
