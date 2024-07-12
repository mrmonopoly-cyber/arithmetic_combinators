use arith_comb_gaph::arith_combinator_graph::{self, ArithOp};

mod arith_comb_gaph;

fn main() {
    let mut graph = arith_combinator_graph::new_graph();
    let zero = arith_combinator_graph::create_op(ArithOp::ZERO);
    let inc = arith_combinator_graph::create_op(ArithOp::INC);

    graph.attach(zero.label);
    graph.attach(inc.label);
    graph.attach(inc.label);

    graph.print_graph();

    graph.copute();
}
