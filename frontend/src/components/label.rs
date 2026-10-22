use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// A label for a form control. Pass `for` in `attrs` to tie it to the control's `id`.
#[component]
pub async fn label(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label
            data-slot="label"
            class=(class!(
                "flex items-center gap-2 text-sm leading-none font-medium select-none \
                 group-data-[disabled=true]:pointer-events-none \
                 group-data-[disabled=true]:opacity-50 \
                 peer-disabled:cursor-not-allowed peer-disabled:opacity-50",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </label>
    })
}
