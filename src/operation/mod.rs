pub mod operations{
    type PortGeneratorFn<'a> = Box<dyn Fn() -> Option<&'a[Option<usize>]>>;

    pub struct Operation<'a> {
        pub arity: usize,
        pub label: &'a str,
        pub generate_aux_ports: PortGeneratorFn<'a>,

        label_id: usize,
    }


    impl<'a> Operation<'a> {
        pub fn zero() -> Self{
            let aux_ports = Box::new(|| None);
            Operation::new(0, "ZERO", 0, aux_ports)
        }

        pub fn same(&self, op: &Operation) -> bool {
            self.label_id == op.label_id
        }

        fn new(arity: usize, name: &'a str, id: usize, gen_f: PortGeneratorFn<'a>) -> Self{
            Self { 
                arity: arity, 
                label: name, 
                label_id: id, 
                generate_aux_ports: gen_f, 
            }
        }
    }
}

