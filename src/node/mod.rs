pub mod graph{
    use std::usize;

    use crate::operation::operations::Operation;

    #[derive(Debug)]
    pub struct Graph<'a>{
        operations: Box<[Operation<'a>]>,
        nodes: Vec<Node>,
        result: Option<usize>,
    }

    #[derive(Debug)]
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
        pub fn new() -> Self{
            Self{
                nodes: Vec::new(),
                operations: Operation::all_operations(),
                result: None,
            }
        }

        pub fn print(&self) {
            for node in self.nodes.iter(){
                let op = &self.operations[node.op_index];
                println!("name: {}", op.label);
                let mut index = 0;
                for port in node.ports.iter(){
                    if let Some(link) = port{
                        let dst_op = &self.operations[link.dst].label;
                        print!("port: {}, ",index);
                        print!("dst: {}, ",dst_op);
                        println!("dst port: {}, ",link.dst_port);
                        println!("============================");
                    }
                    index+=1;
                }
            }
        }

        pub fn attach(&mut self, op: Operation<'a>) {
            let op_index :Option<usize>= {
                let mut res = None;
                let mut index = 0;
                for s_op in self.operations.iter(){
                    if *s_op == op{
                        res = Some(index)
                    }
                    index+=1;
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
