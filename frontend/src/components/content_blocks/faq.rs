use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, class, component, view},
};

use crate::{
    components::{
        lists::ruled::{CLOSED, RULED},
        typography::rich_text::rich_text,
    },
    types::content::FaqItem,
    utils::text::paragraphs,
};

/// A question that opens in place to its answer, with a plus that turns into a cross.
#[component]
async fn faq_item(question: &str, answer: &str) -> Result<impl View> {
    Ok(view! {
        <details data-reveal="" class=(class!(RULED, CLOSED, "group"))>
            <summary class="flex cursor-pointer list-none items-center justify-between gap-6 py-5 text-lg font-bold tracking-tight hover:text-primary-emphasis [&::-webkit-details-marker]:hidden">
                (question)
                icon(data: iconify_icon!("lucide:plus"), attrs: attributes! { class="size-5 shrink-0 text-muted-foreground transition-transform duration-300 group-open:rotate-45" })
            </summary>
            <div class="max-w-4xl pb-6 leading-relaxed text-muted-foreground">
                for paragraph in paragraphs(answer) {
                    <p class="mt-2 first:mt-0">rich_text(text: &paragraph)</p>
                }
            </div>
        </details>
    })
}

/// Questions and their answers, a row each between hairlines.
#[component]
pub async fn faq_list(items: Vec<FaqItem>) -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-col">
            for item in items {
                faq_item(question: &item.question, answer: &item.answer)
            }
        </div>
    })
}
