
pub trait Combinator {
    fn test(&self) -> u8;
}

pub mod generic_combinator{
    use super::Combinator;

    #[derive(Debug,PartialEq)]
    pub struct GenericCombinator<'a> {
        main_port: Option<&'a GenericCombinator<'a>>,
        aux_port: Option<&'a[Option<&'a GenericCombinator<'a>>]>,
    }

    impl <'a> GenericCombinator<'a> {
        pub fn new_no_auxiliary() -> GenericCombinator<'a> {
            GenericCombinator { 
                main_port: None, 
                aux_port: None,
            }
        }

        pub fn new_with_auxiliary(aux_port: &'a [Option<& 'a GenericCombinator<'a>>]) 
            -> GenericCombinator<'a> {
                GenericCombinator { 
                    main_port: None, 
                    aux_port: Some(aux_port),
                }
        }
    }

    impl<'a> Combinator for GenericCombinator<'a>{
        fn test(&self) -> u8 {
            return 1;
        }
    }

}

