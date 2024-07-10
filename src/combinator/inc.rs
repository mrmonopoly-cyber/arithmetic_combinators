pub mod inc_combinator{
    use crate::{combinator::generic_combinator::generic_combinator::BasicCombinator, GenericCombinator};

    
    #[derive(Debug)]
    pub struct IncComb<'a> {
        basic_comb: BasicCombinator<'a>,
    }

    pub fn new<'a>() -> IncComb<'a> {
        IncComb { 
            basic_comb: BasicCombinator::new_no_auxiliary(1,"INC")
        }
    }

    impl <'a> GenericCombinator for IncComb<'a>{
        fn get_lable_id(&self) -> u8 {
            self.basic_comb.get_lable_id()
        }
        fn get_lable_name(&self) -> &str {
            self.basic_comb.get_lable_name()
        }
    }
}
