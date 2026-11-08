//! Long-form content made of blocks, in order: headings, paragraphs, lists, steps,
//! images, galleries, diagrams, questions and answers, code and quotes. Each block is a
//! component of its own, here; this picks the one each block needs. Text keeps a
//! readable line length; images and diagrams take the full width.

use topcoat::{
    Result,
    view::{View, ViewExt, attributes, component, view},
};

use super::{diagram::diagram, typography::section_heading::section_heading};
use crate::types::content::Block;

mod bullets;
mod code;
mod faq;
mod figure;
mod gallery;
mod quote;
mod steps;
mod text;

use bullets::bullet_list;
use code::code_block;
use faq::faq_list;
use figure::figure_block;
use gallery::gallery_block;
use quote::quote_block;
use steps::step_list;
use text::text_block;

#[component]
pub async fn content_blocks(blocks: Vec<Block>) -> Result<impl View> {
    Ok(view! {
        <article class="flex flex-col gap-6">
            for block in blocks {
                match block {
                    Block::Heading { text } => section_heading(attrs: attributes! { data-reveal="" class="mt-6 max-w-3xl" }, (text)),
                    Block::Text { text } => text_block(text: &text),
                    Block::Image { media, caption } => figure_block(src: &media.url, caption: &caption),
                    Block::Gallery { media } => gallery_block(images: media),
                    Block::Code { lang, code } => code_block(lang: &lang, code: &code),
                    Block::List { items } => bullet_list(items: items),
                    Block::Steps { items } => step_list(items: items),
                    Block::Diagram(graph) => diagram(graph: graph),
                    Block::Faq { items } => faq_list(items: items),
                    Block::Quote { text, author } => quote_block(text: &text, author: &author),
                }
            }
        </article>
    }
    // Boxed: a case study nests inside the page deeply enough to overflow the compiler's
    // trait solver otherwise.
    .boxed())
}
