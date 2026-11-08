use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Child, View, class, component, view},
};

use super::form_footer::form_footer;
use crate::components::{
    actions::button::{ButtonShape, ButtonSize, ButtonVariant, button_variants},
    tone::Color,
};

/// The form's last row, sending in two steps: the first press of the button shows `step`
/// (such as a question to answer) above it and turns it into the real send. CSS alone
/// does this: the first button is a label for a hidden checkbox, and the form shows or
/// hides its parts by whether it is checked. With `open`, the step shows from the start,
/// as when the form comes back to have it answered.
#[component]
pub async fn send_step(
    label: &str,
    #[default] open: bool,
    step: Child<'_>,
    note: Child<'_>,
) -> Result<impl View> {
    let look = button_variants(
        Color::default(),
        ButtonShape::Pill,
        ButtonVariant::Inverse,
        ButtonSize::Xl,
    );
    Ok(view! {
        <div class="hidden group-has-[#send-step:checked]/form:block">(step)</div>
        form_footer(
            note: note,
            <span class="contents group-has-[#send-step:checked]/form:hidden">
                <label class=(class!(look.clone(), "has-focus-visible:ring-3 has-focus-visible:ring-white/40"))>
                    <input id="send-step" type="checkbox" class="sr-only" checked=(open.then_some(""))>
                    (label)
                    icon(data: iconify_icon!("lucide:arrow-right"))
                </label>
            </span>
            <span class="hidden group-has-[#send-step:checked]/form:contents">
                <button type="submit" class=(look)>
                    (label)
                    icon(data: iconify_icon!("lucide:arrow-right"))
                </button>
            </span>
        )
    })
}
