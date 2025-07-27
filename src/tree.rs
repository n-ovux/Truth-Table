use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::{self, Debug, Display};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: Weak<RefCell<Self>>,
    children: Vec<Tree<T>>,
}

#[derive(Clone, Debug)]
pub struct Tree<T> {
    head: Rc<RefCell<Node<T>>>,
}

impl<T: Debug + PartialEq + Copy> Tree<T> {
    pub fn new(value: T) -> Self {
        Tree {
            head: Rc::new(RefCell::new(Node {
                value,
                parent: Weak::new(),
                children: Vec::new(),
            })),
        }
    }

    pub fn add_child(&self, value: T) -> Tree<T> {
        let child = Rc::new(RefCell::new(Node {
            value,
            parent: Rc::downgrade(&self.head),
            children: Vec::new(),
        }));

        self.head.borrow_mut().children.push(Tree {
            head: child.clone(),
        });
        Tree {
            head: child.clone(),
        }
    }

    pub fn reparent(&self, new_parent: &Tree<T>) {
        if let Some(parent) = self.head.borrow_mut().parent.upgrade() {
            parent
                .borrow_mut()
                .children
                .retain(|child| !Rc::ptr_eq(&child.head, &self.head))
        }
        self.head.borrow_mut().parent = Rc::downgrade(&new_parent.head);
        new_parent.head.borrow_mut().children.push(Tree {
            head: Rc::clone(&self.head),
        })
    }

    pub fn replace_if_match(&mut self, target: T, value: T) {
        if self.head.borrow().value == target {
            self.head.borrow_mut().value = value;
            self.head.borrow_mut().children.clear();
        }
    }

    pub fn find_replace(&mut self, target: T, value: T) {
        self.replace_if_match(target, value);
        for child in &mut self.head.borrow_mut().children {
            child.find_replace(target, value);
        }
    }

    fn write(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        writeln!(f, "{}{:?}", "  ".repeat(depth), self.head.borrow().value)?;

        for child in &self.head.borrow().children {
            child.write(f, depth + 1)?;
        }
        Ok(())
    }
}

impl<T: Debug + PartialEq + Copy> Display for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        self.write(f, 0)
    }
}
