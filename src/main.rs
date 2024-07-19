use arith_comb_gaph::arith_combinator_graph::{self, ArithOp};

mod arith_comb_gaph;

fn main() {
    let graph = Box::leak(Box::new(arith_combinator_graph::new_graph()));
    let zero = arith_combinator_graph::create_op(ArithOp::ZERO);
    let inc = arith_combinator_graph::create_op(ArithOp::INC);
    let pos = arith_combinator_graph::create_op(ArithOp::POS);
    let sum = arith_combinator_graph::create_op(ArithOp::SUM);

    graph.attach(zero.label);
    graph.attach(inc.label);
    graph.attach(inc.label);
    // graph.attach(pos.label);
    // graph.attach(inc.label);

    graph.print_graph();

    graph.compute();
}
