pub mod graph{
    use std::usize;

    use crate::operation::operations::Operation;

    pub struct Graph<'a>{
        operations: Vec<Operation<'a>>,
        nodes: Vec<Node<'a>>,
        result: Option<usize>,
    }

    struct Node<'a> {
        pub index_op: usize,
        pub main_port: Option<usize>,
        pub aux_port: Option<&'a [Option<usize>]>
    }

    impl<'a> Graph<'a> {
        pub fn attach(& mut self, new_node: &Operation<'a>){
            let mut op : Option<usize> = None;
            for i in 0..self.operations.len(){
                if self.operations[i].same(new_node) {
                    op = Some(i);
                }
            }
            let new_node : Option<Node> = match op {
                None => None,
                Some(op) => {
                    let ope = &self.operations[op];
                    let aux_ports = (ope.generate_aux_ports)();
                    Some(Node{
                        main_port : None,
                        index_op : op,
                        aux_port : aux_ports,
                    })
                },
            };

            if let Some(node) = new_node{
                match &self.result {
                    None => {
                        self.nodes.push(node);
                        self.result = Some(0);
                    }
                    Some(_r) => {
                    },
                }
            }
        }

        pub fn print(& self){
            match &self.result {
                None => {},
                Some(n) => {
                    let res_node = &self.nodes[n.clone()];
                    let node_name = &self.operations[res_node.index_op].label;
                    let main_port_node_name = {
                        match res_node.main_port{
                            None => "NONE",
                            Some(i) =>{
                                &self.operations[i].label
                            }
                        }
                    };
                    println!("node label: {}", node_name);
                    println!("main port: {}", main_port_node_name);
                    if let Some(aux) = res_node.aux_port{
                        for i in 0..aux.len(){
                            let aux_port_name = {
                                match aux[i]{
                                    None => "NONE",
                                    Some(i) =>{
                                        &self.operations[i].label
                                    }
                                }
                            };
                            println!("port {}: {} ",i,aux_port_name);
                        }
                    }
                },
            }
        }

        pub fn new() -> Self {
            Graph { 
                operations: vec![Operation::zero()],
                nodes: Vec::new(),
                result: None,
            }
        }
    }


    }
