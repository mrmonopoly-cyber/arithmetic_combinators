mod node;
mod operation;

use node::graph::Graph;
use operation::operations::Operation;

fn main() {

    let mut graph = Graph::new();
    graph.attach(Operation::zero());
    graph.attach(Operation::inc());
    graph.attach(Operation::sum());

    graph.print();
}
