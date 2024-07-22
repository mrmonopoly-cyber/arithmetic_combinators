mod arith_comb_gaph;

use arith_comb_gaph::arith_combinator_graph::{compute, get_result, print_graph, push_num, push_op, reset};

fn main() {
    push_op('+');
    push_op('+');
    push_num(12);
    push_num(15);
    push_op('+');
    push_op('+');
    push_num(19);
    push_num(37);
    push_op('+');
    push_num(71);
    push_num(68);

    print_graph();
    compute();
    print_graph();

    match get_result(){
        None => println!("computation failed"),
        Some(r) => println!("res = {}",r),
    }

    reset();
}
