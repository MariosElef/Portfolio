use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// Two things at either end of a row, aligned at the top: such as a project's mark on the
/// left and its name on the right.
#[component]
pub async fn spread(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="flex items-start justify-between gap-6">(child)</div>
    })
}
