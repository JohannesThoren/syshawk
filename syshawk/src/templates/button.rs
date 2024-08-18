use syshawk_templating::html;
use syshawk_templating::node::Node;

pub fn button(text_content: String) -> Node {
    return html::button(text_content)
        .class("bg-sky-600 px-2 py-1 rounded-sm text-white hover:bg-sky-400".to_string());
}