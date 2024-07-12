pub mod graph{
    use std::usize;

    use crate::arith_comb_gaph::operation::operations::Operation;

    use super::super::operation_pool::operation_pool::*;

    #[derive(Debug)]
    pub struct Graph<'a>{
        operations: OpPool<'a>,
        nodes: Vec<Node<'a>>,
        result: Option<usize>,
    }

    #[derive(Debug,Clone)]
    pub struct Link {
        dst: usize,
        dst_port: usize,
    }

    #[derive(Debug)]
    struct Node<'a>{
        op_label : &'a str,
        pub ports: Box<[Option<Link>]>,
        //0 is return port
        //1 is main port if possible, else 0 is main port
    }

    impl<'a> Node<'a> {
        fn new(op: &Operation<'a>) -> Self{
            Self{
                op_label: op.label,
                ports:{
                    let none_ports: Option<Link> = None;
                    let vec_ports = vec![none_ports;op.arity];
                    vec_ports.into_boxed_slice()
                },
            }
        }

        fn free_port(&self) -> Option<usize> {
            let mut res = None;
            let mut index =0;
            for i in self.ports.iter(){
                if let None = i{
                    res = Some(index);
                }
                index+=1;
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
        pub fn new(ops: OpPool<'a>) -> Self{
            Self{
                operations: ops,
                nodes: Vec::new(),
                result: None,
            }
        }

        pub fn print_graph(&self) {
            println!("GRAPH:========================");
            for node in self.nodes.iter(){
                println!("name: {}", node.op_label);
                let mut index = 0;
                for port in node.ports.iter(){
                    if let Some(link) = port{
                        let dst_node = &self.nodes[link.dst];
                        print!("port: {}, ",index);
                        print!("dst: {}, ", self.operations.find(dst_node.op_label).unwrap().label);
                        println!("dst port: {}, ",link.dst_port);
                        println!("============================");
                    }
                    index+=1;
                }
            }

            self.operations.print_rules();
        }

        pub fn attach(&mut self, op_name: &'a str) {
            // let op_index :Option<usize>= self.find_op_index(&op);
            let op = self.operations.find(op_name);
            match op{
                None => (),
                Some(op) =>{
                    let new_node = Node::new(op);
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
                },
            }
        }


        pub fn copute(&mut self){
            if let Some(res) = self.result{
                let _cursor = res;
            }
        }
        
    }
}
