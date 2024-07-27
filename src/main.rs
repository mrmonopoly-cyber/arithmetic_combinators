mod arith_comb_gaph;

use arith_comb_gaph::arith_combinator_graph::{compute, get_result, print_graph, print_rules, push_num, push_op, reset};

fn main() {
    push_op('/');
    push_num(7);
    push_num(5);

    print_graph();
    compute();
    print_graph();


    match get_result(){
        None => println!("computation failed"),
        Some(r) => println!("res = {}",r),
    }

    reset();
}
