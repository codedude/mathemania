use crate::graph::{Graph, Node};
use crate::sudoku::Sudoku;

pub mod graph;
pub mod sudoku;

fn main() {
    println!("Hello, world!");
    let sudoku = Sudoku::default();
    sudoku.print();

    let mut graph = Graph::<i32>::default();
    graph.add_vertex(Node::new(1));
    graph.add_vertex(Node::new(2));
    graph.add_vertex(Node::new(3));
    graph.print();
}
