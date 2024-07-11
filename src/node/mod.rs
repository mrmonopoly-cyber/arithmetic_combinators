pub mod graph{
    use std::usize;

    use crate::operation::operations::Operation;

    #[derive(Debug)]
    pub struct Graph<'a>{
        operations: Box<[Operation<'a>]>,
        nodes: Vec<Node>,
        result: Option<usize>,
    }

    #[derive(Debug,Clone)]
    pub struct Link {
        dst: usize,
        dst_port: usize,
    }

    #[derive(Debug)]
    struct Node{
        op_index: usize,
        pub ports: Box<[Option<Link>]>,
        //0 is return port
        //1 is main port if possible, else 0 is main port
    }

    impl Node {
        fn free_port(&self) -> Option<usize> {
            let mut res = None;
            println!("se, {}",self.ports.len());
            for i in (self.ports.len()-1)..=0{
                println!("se in , {}",i);
                if let None = self.ports[i]{
                    res = Some(i);
                }
            };
            res
        }

        fn link_to(&mut self, dst_node: usize,port :usize, dst_node_port: usize){
            self.ports[port] = Some(
                Link{
                    dst: dst_node,
                    dst_port: dst_node_port,
                }
            );
        }
        
    }
    
    impl<'a> Graph<'a> {
        pub fn new() -> Self{
            Self{
                nodes: Vec::new(),
                operations: Operation::all_operations(),
                result: None,
            }
        }

        pub fn print(&self) {
            fn print_node(s :&Graph, n_index: usize){
                let res = &s.nodes[n_index];
                let op = &s.operations[res.op_index];

                for p_index in (res.ports.len()-1)..0{
                    if let Some(link) = &res.ports[p_index]  {
                        print_node(&s, link.dst_port);
                    }
                }
                println!("name: {}", op.label);
                for port in (op.arity-1)..=0{
                    if let Some(link) = &res.ports[port]{
                        let dst_op = &s.operations[link.dst].label;
                        println!("start port: {}, ",port);
                        println!("dst port: {}, ",link.dst_port);
                        println!("dst : {}, ",dst_op);
                    }
                }
            }

            if let Some(res) = self.result{
                print_node(self, res);
            }
        }

        pub fn attach(&mut self, op: Operation<'a>) {
            let op_index :Option<usize>= {
                let mut res = None;
                for s_op_i in 0..self.operations.len(){
                    if self.operations[s_op_i] == op{
                        res = Some(s_op_i)
                    }
                };
                res
            };

            let new_node = Node{
                ports : op.generate_ports(),
                op_index: op_index.unwrap(),
            };
            let new_node_free_port = new_node.free_port().unwrap();
            let new_node_index = self.nodes.len();
            self.nodes.push(new_node);

            match self.result {
                None =>{
                    return self.result = Some(0);
                },
                Some(rn) =>{
                    let res_node : &mut Node = &mut self.nodes[rn];
                    let res_free_port = res_node.free_port().unwrap();

                    res_node.link_to(new_node_index, res_free_port, 
                        new_node_free_port);
                    self.nodes[new_node_index]
                        .link_to(rn, new_node_free_port,res_free_port);
                    if res_free_port == 0{
                        self.result = Some(new_node_index);
                    }
                },
            }
        }
        
    }

}
