mod combinator{
    #[derive(Debug,PartialEq)]
    pub struct Combinator{
        prim_port: Box<Option<Combinator>>,
        aux_port: Option<Vec<Option<Box<Combinator>>>>,
    }

    impl Combinator {
        pub fn new(num_aux_port: u8) ->Combinator {
            
            let mut aux_port_maybe = None;
            if num_aux_port > 0{
                let mut aux_port : Vec<Option<Box<Combinator>>> = Vec::new();
                for _ in 0..num_aux_port{
                    aux_port.push(None);
                }
                aux_port_maybe = Some(aux_port);
            }

            Combinator{
                prim_port: Box::new(None),
                aux_port: aux_port_maybe,
            }
        }
        
    }
}


fn main() {
    let comb = combinator::Combinator::new(0);
}
