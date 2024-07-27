mod arith_comb_gaph;

use arith_comb_gaph::arith_combinator_graph::*;

fn main() {
    push_op('/');
    push_num(-10);
    push_num(2);

    print_graph();
    compute();
    print_graph();


    match get_result(){
        None => println!("computation failed"),
        Some(r) => println!("res = {}",r),
    }

    reset();
}
