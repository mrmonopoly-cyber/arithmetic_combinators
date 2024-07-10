pub mod dec_combinator{
    use crate::{combinator::generic_combinator::generic_combinator::BasicCombinator, GenericCombinator};

    
    #[derive(Debug)]
    pub struct DecComb<'a> {
        basic_comb: BasicCombinator<'a>,
    }

    pub fn new<'a>() -> DecComb<'a> {
        DecComb { 
            basic_comb: BasicCombinator::new_no_auxiliary(2,"DEC")
        }
    }

    impl <'a> GenericCombinator for DecComb<'a>{
        fn get_lable_id(&self) -> u8 {
            self.basic_comb.get_lable_id()
        }
        fn get_lable_name(&self) -> &str {
            self.basic_comb.get_lable_name()
        }
    }
}
