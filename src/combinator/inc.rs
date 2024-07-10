pub mod inc_combinator{
    use crate::combinator::generic_combinator::basic_combinator::BasicCombinator;

    pub fn new_inc<'a>() -> BasicCombinator<'a> {
        BasicCombinator::new_basic_combinator(1,"INC",None,None)
    }
}
