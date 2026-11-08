use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

const CARD: StaticClass = class!(
    "flex flex-col gap-6 rounded-xl border border-border bg-card py-6 text-card-foreground \
     shadow-sm",
);

/// A bordered panel that groups related content.
#[component]
pub async fn card(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="card" class=(class!(CARD, attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

/// The opening section of a [`card`].
#[component]
pub async fn card_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="card-header"
            class=(class!(
                "grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 \
                 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The heading of a [`card`], rendered as an `<h3>`.
#[component]
pub async fn card_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <h3
            data-slot="card-title"
            class=(class!("leading-none font-semibold", attrs.remove("class")))
            (attrs)
        >
            (child)
        </h3>
    })
}

/// The supporting text under a [`card_title`].
#[component]
pub async fn card_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="card-description"
            class=(class!("text-sm text-muted-foreground", attrs.remove("class")))
            (attrs)
        >
            (child)
        </p>
    })
}

/// An action in the top right corner of a [`card_header`].
#[component]
pub async fn card_action(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="card-action"
            class=(class!(
                "col-start-2 row-span-2 row-start-1 self-start justify-self-end",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The main body of a [`card`].
#[component]
pub async fn card_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="card-content" class=(class!("px-6", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

/// The closing section of a [`card`], a horizontal row for actions.
#[component]
pub async fn card_footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="card-footer"
            class=(class!("flex items-center px-6 [.border-t]:pt-6", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}
