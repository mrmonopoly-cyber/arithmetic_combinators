use super::generic_combinator::basic_combinator::BasicCombinator;

pub type CombGraph<'a> = &'a [&'a BasicCombinator<'a>];
