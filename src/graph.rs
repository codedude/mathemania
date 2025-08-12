use std::fmt::Display;

pub struct Graph<'a, T: PartialOrd + Display> {
    vertices: Vec<Box<Node<'a, T>>>,
}

pub struct Node<'a, T: PartialEq + Display> {
    edges: Vec<&'a Box<Node<'a, T>>>,
    value: T,
}

impl<'a, T: PartialOrd + Display> Node<'a, T> {
    pub fn new(value: T) -> Box<Node<'a, T>> {
        Box::new(Node {
            edges: vec![],
            value,
        })
    }
}

impl<'a, T: PartialOrd + Display> Graph<'a, T> {
    pub fn default() -> Graph<'a, T> {
        Graph { vertices: vec![] }
    }

    pub fn add_vertex(&mut self, node: Box<Node<'a, T>>) {
        self.vertices.push(node);
    }

    pub fn print(&self) {
        for vertex in self.vertices.iter() {
            print!("{}:\t", vertex.value);
            for edge in vertex.edges.iter() {
                print!("{}, ", edge.value);
            }
            println!();
        }
        println!();
    }
}
