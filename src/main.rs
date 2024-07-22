mod arith_comb_gaph;

use arith_comb_gaph::arith_combinator_graph::{compute, get_result, print_graph, push_num, push_op, reset};

fn main() {
    // push_op('+');
    // push_num(1);
    // push_num(2);

    push_op('+');
    push_op('+');
    push_num(1);
    push_num(2);
    push_op('+');
    push_num(3);
    push_num(2);

    print_graph();
    compute();

    match get_result(){
        None => println!("computation failed"),
        Some(r) => println!("res = {}",r),
    }

    reset();
}
