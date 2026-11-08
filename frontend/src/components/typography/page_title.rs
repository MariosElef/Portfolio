use topcoat::{
    Result,
    view::{Child, StaticClass, View, class, component, view},
};

/// How large a [`page_title`] is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TitleSize {
    /// A page of content, such as a list or a form.
    #[default]
    Md,
    /// A page about one thing, such as a project.
    Lg,
    /// A page's own hero, such as About.
    Display,
}

impl TitleSize {
    fn classes(self) -> StaticClass {
        match self {
            Self::Md => class!("text-4xl font-bold tracking-tight"),
            Self::Lg => class!("text-5xl leading-none font-bold tracking-tight sm:text-7xl"),
            Self::Display => class!("text-5xl font-bold tracking-tighter sm:text-7xl"),
        }
    }
}

/// The page's one `<h1>`.
#[component]
pub async fn page_title(
    #[default] size: TitleSize,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <h1 class=(size.classes())>(child)</h1>
    })
}
