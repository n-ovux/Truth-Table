use std::cmp::PartialEq;
use std::fmt::{self, Debug, Display};

#[derive(Clone, Debug)]
pub struct Tree<T> {
    vertices: Vec<T>,
    edges: Vec<(usize, usize)>,
}

impl<T: Debug + PartialEq + Copy> Tree<T> {
    pub fn new(value: T) -> Self {
        Tree {
            vertices: vec![value],
            edges: Vec::new(),
        }
    }

    pub fn add_child(&mut self, index: usize, value: T) -> usize {
        self.vertices.push(value);
        self.edges.push((index, self.vertices.len() - 1));
        self.vertices.len() - 1
    }

    pub fn reparent(&mut self, child: usize, new_parent: usize) {
        for edge in &mut self.edges {
            if edge.1 == child {
                edge.0 = new_parent;
            }
        }
    }

    pub fn find_replace(&mut self, target: T, value: T) {
        for (index, vertex) in self.vertices.iter_mut().enumerate() {
            if *vertex == target {
                *vertex = value;
                self.edges.retain(|edge| edge.0 != index)
            }
        }
    }

    pub fn get_vertices(&self) -> &Vec<T> {
        &self.vertices
    }

    pub fn get_edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }

    fn write(&self, f: &mut fmt::Formatter<'_>, index: usize, depth: usize) -> fmt::Result {
        writeln!(f, "{}{:?}", "  ".repeat(depth), self.vertices[index])?;
        let mut children: Vec<usize> = Vec::new();
        for edge in &self.edges {
            if edge.0 == index {
                children.push(edge.1);
            }
        }

        for child in children {
            self.write(f, child, depth + 1)?;
        }

        Ok(())
    }
}

impl<T: Debug + PartialEq + Copy> Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f, 0, 0)
    }
}
