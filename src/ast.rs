#[derive(Debug, PartialEq)]
pub enum Token {
    VARIABLE,
    BRACKET,
    OPERATOR,
    CONSTANT,
    NEGATION,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Grammar {
    ROOT,
    VALUE,
    OPERATOR,
}

impl std::fmt::Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Node {
    pub value: Grammar,
    pub children: Vec<Node>,
}

impl Node {
    pub fn add_child(&mut self, child: Grammar) {
        self.children.push(Node::new(child));
    }

    pub fn new(node_type: Grammar) -> Self {
        Self {
            value: node_type,
            children: Vec::new(),
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.children.is_empty() {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{}( ", self.value).unwrap();
            for child in &self.children {
                write!(f, " {}, ", child).unwrap();
            }
            write!(f, " )")
        }
    }
}
