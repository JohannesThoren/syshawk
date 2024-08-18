use syshawk_templating::html::{div, p};
use syshawk_templating::node::Node;

pub fn progress_bar(percent: f32) -> Node {
    div(vec![
        div(vec![
            p(format!("{:.2}%", percent))
        ])
            .class("bg-blue-600 h-4 rounded-sm flex text-xs px-2".to_string())
            .style(format!("width: {}%; justify-content: right", percent))
    ]).class("w-full bg-gray-200 rounded-sm h-4".to_string())
}