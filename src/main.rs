mod combinator;

use combinator::generic_combinator::{self, GenericCombinator};

use crate::combinator::Combinator;

fn main() {
    let aux_port: &[Option<&GenericCombinator>] = &[None,None];

    let _comb_no_aux = generic_combinator::GenericCombinator::new_no_auxiliary();
    let _comb_aux = generic_combinator::GenericCombinator::new_with_auxiliary(aux_port);
    _comb_no_aux.test();
}
