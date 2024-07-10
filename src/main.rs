pub mod combinator;

use combinator::*;

fn main() {
    let zero_c = zero::zero_combinator::new();
    let inc_c = inc::inc_combinator::new();
    let dec_c = dec::dec_combinator::new();

    println!("{},{}",zero_c.get_lable_id(),zero_c.get_lable_name());
    println!("{},{}",inc_c.get_lable_id(),inc_c.get_lable_name());
    println!("{},{}",dec_c.get_lable_id(),dec_c.get_lable_name());

}
