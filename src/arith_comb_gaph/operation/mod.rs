pub mod operations{
    #[derive(Debug,PartialEq)]
    pub struct Operation<'a> {
        pub label: &'a str,
        pub arity: usize,
    }


    impl<'a> Operation<'a> {
        pub fn new(arity: usize, name: &'a str) -> Self{
            Self { 
                label: name, 
                arity: arity,
            }
        }
    }
}

