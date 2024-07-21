use arith_comb_gaph::arith_combinator_graph::{compute, print_graph, push_op, reset, ArithOp};

mod arith_comb_gaph;

fn main() {

    push_op(ArithOp::SUM);
    push_op(ArithOp::POS);
    push_op(ArithOp::ZERO);


    print_graph();
    compute();

    reset();
}
