pub mod generic_combinator{
    use crate::combinator::GenericCombinator;


    #[derive(Debug)]
    pub struct BasicCombinator<'a> {
        name: &'a str,
        label: u8,
        main_port: Option<&'a BasicCombinator<'a>>,
    }

    impl <'a> BasicCombinator<'a> {
        pub fn new_no_auxiliary(label: u8, name: &'a str) 
            -> BasicCombinator<'a> {
            BasicCombinator { 
                name: name,
                label: label,
                main_port: None, 
            }
        }
    }

    impl <'a> GenericCombinator for BasicCombinator<'a>{
        fn get_lable_id(&self) -> u8 {
            self.label
        }

        fn get_lable_name(&self) -> &str {
            self.name
        }
    }
}
