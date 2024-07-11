pub mod operations{
    use crate::node::graph::Link;

    #[derive(Debug,PartialEq)]
    pub struct Operation<'a> {
        pub label: &'a str,
        pub arity: usize,
    }


    impl<'a> Operation<'a> {
        pub fn all_operations() -> Box<[Operation<'a>]>{
            let op_vec = Vec::from([
                Operation::zero(),
                Operation::inc(),
            ]);
            op_vec.into_boxed_slice()
        }

        pub fn zero() -> Self{
            Operation::new(1,"ZERO")
        }

        pub fn inc() -> Self{
            Operation::new(2,"INC")
        }

        pub fn generate_ports(&self) -> Box<[Option<Link>]>{
            let mut aux_vec  = Vec::new();
            for _ in 0..self.arity {
                aux_vec.push(None);
            }

            aux_vec.into_boxed_slice()
        }

        fn new(arity: usize, name: &'a str) -> Self{
            Self { 
                label: name, 
                arity: arity,
            }
        }
    }
}

