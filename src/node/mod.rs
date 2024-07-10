pub mod node{
    #[derive(Debug,Clone)]
    pub struct Node<'a> {
        //0 : main port
        //1..arity-2: aux_port
        //arity-1: Result port
        ports: Vec<Option<Node<'a>>>,
        arity: usize,
        next_free: usize,
        label: &'a str,
    }
    
    impl<'a> Node<'a> {
        pub fn zero() -> Node<'a> {
            Node::new("ZERO", 1)
        }

        pub fn inc() -> Node<'a> {
            Node::new("INC", 2)
        }

        pub fn dec() -> Node<'a> {
            Node::new("DEC", 2)
        }

        pub fn sum() -> Node<'a> {
            Node::new("SUM", 3)
        }

        pub fn print_tree(&self){
            println!("node {}",self.label);
            for i in  0..self.arity {
                if let Some(n) = &self.ports[i]{
                    println!("{} port {} : {}",self.label,i,n.label);
                    n.print_tree();
                }
            }
        }

        pub fn attach(mut self, mut new_node: Node<'a>) -> Node<'a>{
            match self.next_free {
                0 =>{
                    new_node.ports[new_node.arity-1] = Some(self);
                    new_node
                },
                _ =>{
                    self.ports[self.next_free] = Some(new_node);
                    self.next_free-=1;
                    self
                },
            }
        }

        fn new(name: &str, arity: usize) -> Node{
            Node{
                arity: arity,
                next_free: {
                    match arity {
                        1 => 0,
                        _ => arity -2,
                    }
                },
                label: name,
                ports:  vec![None;arity],
            }
        }
    }

}
