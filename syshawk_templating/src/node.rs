use crate::attribute::Attribute;

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) tag: String,
    pub(crate) class: Option<String>,
    pub(crate) id: Option<String>,
    pub(crate) style: Option<String>,
    pub(crate) attributes: Option<Vec<Attribute>>,
    pub(crate) children: Option<Vec<Node>>,
    pub(crate) text_content: Option<String>,
}

impl Node {
    pub fn new(tag: String, class: Option<String>, id: Option<String>, style: Option<String>, attributes: Option<Vec<Attribute>>, children: Option<Vec<Node>>, text_content: Option<String>) -> Node {
        Node {
            tag,
            class,
            id,
            style,
            attributes,
            children,
            text_content,
        }
    }

    pub fn class(mut self, class: String) -> Self {
        self.class = Some(class);
        self
    }
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }
    pub fn style(mut self, style: String) -> Self {
        self.style = Some(style);
        self
    }
    pub fn attributes(mut self, attributes: Vec<Attribute>) -> Self {
        self.attributes = Some(attributes);
        self
    }
    pub fn children(mut self, children: Vec<Node>) -> Self {
        self.children = Some(children);
        self
    }

    pub fn text_content(mut self, text_content: String) -> Self {
        self.text_content = Some(text_content);
        self
    }

    pub fn to_html_string(self) -> String {
        let mut html: String = String::from("");
        html.push_str(format!("<{}", self.tag).as_str());

        if self.class.is_some() { html.push_str(format!(" class=\"{}\"", self.class.unwrap()).as_str()) }

        if self.id.is_some() { html.push_str(format!(" id=\"{}\"", self.id.unwrap()).as_str()) }

        if self.style.is_some() { html.push_str(format!(" style=\"{}\"", self.style.unwrap()).as_str()) }

        if self.attributes.is_some() {
            for attribute in self.attributes.unwrap() {
                html.push_str(format!(" {}=\"{}\"", attribute.key, attribute.value).as_str())
            }
        }

        html.push_str(">");

        if self.children.is_some() {
            for child in self.children.unwrap() {
                html.push_str(child.to_html_string().as_str())
            }
        }
        if self.text_content.is_some() {
            html.push_str(self.text_content.unwrap().as_str())
        }

        html.push_str(format!("</{}>", self.tag).as_str());
        return html;
    }
}

