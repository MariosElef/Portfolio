use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// A form's last row: small print on the left and the button on the right, the button
/// first and full width on a phone.
#[component]
pub async fn form_footer(note: Child<'_>, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="mt-2 flex flex-col-reverse gap-4 border-t border-white/10 pt-6 sm:flex-row sm:items-center sm:justify-between">
            <div class="min-w-0">(note)</div>
            <div class="flex [&>*]:w-full sm:[&>*]:w-auto">(child)</div>
        </div>
    })
}
