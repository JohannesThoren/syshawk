use crate::node::Node;

pub fn div(children: Vec<Node>) -> Node {
    Node {
        tag: "div".to_string(),
        class: None,
        id: None,
        style: None,
        attributes: None,
        children: Some(children),
        text_content: None,
    }
}

pub fn h(heading_size: usize, text_content: String) -> Node {
    Node {
        tag: format!("h{}", heading_size),
        class: None,
        id: None,
        style: None,
        attributes: None,
        children: None,
        text_content: Some(text_content),
    }
}

pub fn p(text_content: String) -> Node {
    Node {
        tag: "p".to_string(),
        class: None,
        id: None,
        style: None,
        attributes: None,
        children: None,
        text_content: Some(text_content),
    }
}

pub fn header(children: Vec<Node>) -> Node {
    Node {
        tag: "header".to_string(),
        class: None,
        id: None,
        style: None,
        attributes: None,
        children: Some(children),
        text_content: None,
    }
}

pub fn button(text_content: String) -> Node {
    Node {
        tag: "button".to_string(),
        class: None,
        id: None,
        style: None,
        attributes: None,
        children: None,
        text_content: Some(text_content),
    }
}