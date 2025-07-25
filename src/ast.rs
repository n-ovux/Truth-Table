use std::cell::RefCell;
use std::rc::{Rc, Weak};

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Grammar {
    ROOT,
    VALUE(char),
    VARIABLE(char),
    OPERATOR(char),
    NEGATION,
}

impl std::fmt::Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Node {
    value: Grammar,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(value: Grammar) -> Self {
        Node {
            value,
            parent: None,
            children: Vec::new(),
        }
    }
    pub fn add_child(self_rc: &Rc<RefCell<Node>>, value: Grammar) -> Rc<RefCell<Node>> {
        let child = Rc::new(RefCell::new(Node {
            value,
            parent: Some(Rc::downgrade(self_rc)),
            children: Vec::new(),
        }));

        self_rc.borrow_mut().children.push(child.clone());
        child.clone()
    }

    pub fn replace_if_match(&mut self, target: Grammar, value: Grammar) {
        if self.value == target {
            self.value = value;
            self.children.clear();
        }
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        self.parent.as_ref().and_then(|weak| weak.upgrade())
    }

    pub fn get_children(&self) -> &Vec<Rc<RefCell<Node>>> {
        &self.children
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.children.is_empty() {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{}( ", self.value).unwrap();
            for child in &self.children {
                write!(f, "{}, ", child.borrow()).unwrap();
            }
            write!(f, " )")
        }
    }
}
