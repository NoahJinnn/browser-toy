use std::collections::{HashMap, HashSet};

pub type AttrMap = HashMap<String, String>;

pub struct Node {
    // data common to all nodes:
    children: Vec<Node>,
    // data specific to each node type:
    n_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attr: AttrMap,
}

// Node constructors
pub fn text(data: &str) -> Node {
    Node {
        children: Vec::new(),
        n_type: NodeType::Text(data.to_owned()),
    }
}

pub fn element(name: &str, attr: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        n_type: NodeType::Element(ElementData {
            tag_name: name.to_owned(),
            attr,
        }),
    }
}

pub fn printDOMTree(node: Node, tab: &str) {
    match node.n_type {
        NodeType::Text(data) => {
            println!("{}", format!("{tab}{data}"));
        }
        NodeType::Element(ele) => {
            let tag_name = ele.tag_name;
            let open = format!("{tab}<{tag_name}>");
            println!("{open}");
            for node in node.children {
                printDOMTree(node, tab);
            }
            let close = format!("{tab}</{tag_name}>");
            println!("{close}");
        }
    }
}

// Element methods

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attr.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attr.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new()
        }
    }
}
