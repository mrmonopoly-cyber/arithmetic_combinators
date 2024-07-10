pub mod zero_combinator{
    use crate::graph::generic_combinator::basic_combinator::BasicCombinator;

    pub fn new_zero_com<'a>() -> BasicCombinator<'a> {
        BasicCombinator::new("ZERO", 0, None)
    }   
}

