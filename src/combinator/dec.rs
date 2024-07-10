pub mod dec_combinator{
    use crate::combinator::generic_combinator::basic_combinator::BasicCombinator;

    pub fn new_dec<'a>() -> BasicCombinator<'a> {
        BasicCombinator::new_basic_combinator(2,"DEC",None,None)
    }
}
