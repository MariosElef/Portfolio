use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// A form posted to its own page, its fields one above another.
#[component]
pub async fn form(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <form class="group/form flex flex-col gap-5" method="post">(child)</form>
    })
}
