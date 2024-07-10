pub mod zero_combinator{
    use crate::combinator::generic_combinator::basic_combinator::BasicCombinator;

    pub fn new_zero<'a>() -> BasicCombinator<'a>{
        BasicCombinator::new_basic_combinator(0,"ZERO",None,None)
    }
}

