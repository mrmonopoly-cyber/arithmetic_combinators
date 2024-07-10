pub mod zero_combinator{
    use crate::combinator::{generic_combinator::generic_combinator::BasicCombinator, GenericCombinator};

    #[derive(Debug)]
    pub struct ZeroComb<'a> {
        basic_comb: BasicCombinator<'a>,
    }

    pub fn new<'a>() -> ZeroComb<'a>{
        ZeroComb{
            basic_comb: BasicCombinator::new_no_auxiliary(0,"ZERO"),
        }
    }

    impl<'a> GenericCombinator for ZeroComb<'a>{
        fn get_lable_id(&self) -> u8 {
            self.basic_comb.get_lable_id()
        }
        fn get_lable_name(&self) -> &str {
            self.basic_comb.get_lable_name()
        }
    }
}

