mod node;

use node::node::Node;

fn main() {

    let graph = Node::inc();
    let graph = graph.attach(Node::inc());
    let graph = graph.attach(Node::zero());
    let graph = graph.attach(Node::sum());

    graph.print_tree();
}
