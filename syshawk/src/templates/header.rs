use syshawk_templating::html::{div, h, header};
use crate::templates::button::button;

pub fn header_template() -> String {
    header(vec![
        h(1, "SYSHawk".to_string()),
        div(vec![
            button("add probe".to_string()),
        ])
            .class("flex gap-2".to_string())
            .id("controls".to_string()),
    ])
        .class("px-5 py-2 bg-gray-400 flex justify-between items-center h-full".to_string())
        .to_html_string()
}