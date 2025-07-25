use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq)]
pub enum Token {
    VALUE,
    BRACKET,
    OPERATOR,
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

    pub fn find_replace(&mut self, target: Grammar, value: Grammar) {
        self.replace_if_match(target, value);
        for child in &self.children {
            child.borrow_mut().replace_if_match(target, value);
        }
    }

    pub fn evaluate(&mut self) -> bool {
        if self.children.is_empty() {
            todo!()
        } else {
            for child in &self.children {
                child.borrow_mut().evaluate();
            }
        }
        false
    }

    fn write(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) {
        writeln!(f, "{}{}", "  ".repeat(depth), self.value).unwrap();

        for child in &self.children {
            child.borrow().write(f, depth + 1);
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write(f, 0);
        Ok(())
    }
}
