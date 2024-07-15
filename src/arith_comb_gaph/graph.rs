pub mod graph{
    use core::panic;
    use std::{usize, vec};

    use crate::arith_comb_gaph::operation::operations::Operation;

    use super::super::operation_pool::operation_pool::*;

    #[derive(Debug)]
    pub struct Graph<'a>{
        operations: OpPool<'a>,
        nodes: Vec<Node<'a>>,
        links: Vec<Link>,
        result: Option<usize>,
    }

    #[derive(Debug,Clone)]
    pub struct Link {
        start: usize,
        dst: usize,
        start_port: usize,
        dst_port: usize,
    }

    #[derive(Debug)]
    struct Node<'a>{
        op_label : &'a str,
        pub ports: Box<[Option<usize>]>,
        main_port: usize,
        //0 is return port
        //1 is main port if possible, else 0 is main port
    }

    impl<'a> Node<'a> {
        fn new(op: &Operation<'a>) -> Self{
            Self{
                op_label: op.label,
                ports:{
                    let none_ports: Option<usize> = None;
                    let vec_ports = vec![none_ports;op.arity];
                    vec_ports.into_boxed_slice()
                },
                main_port: match op.arity {
                    1 => 0,
                    _ => 1,
                },
            }
        }

        fn free_port(&self) -> Option<usize> {
            let mut res = None;
            let mut index = self.ports.len()-1;
            for i in self.ports.iter().rev(){
                if let None = i{
                    res = Some(index);
                    break;
                }
                if index > 0{
                    index-=1;
                }
            };
            res
        }

        
    }
    
    impl<'a> Graph<'a> {
        pub fn new(ops: OpPool<'a>) -> Self{
            Self{
                operations: ops,
                nodes: Vec::new(),
                links: Vec::new(),
                result: None,
            }
        }

        fn add_link(&mut self,start_node: usize, dst_node: usize,start_port :usize, dst_node_port: usize){
            self.links.push(
                Link{
                    start: start_node,
                    dst: dst_node,
                    start_port: start_port,
                    dst_port: dst_node_port,
                }
            );
        }

        fn get_node_linked_to(&self, start_node: usize, link: &Link) -> usize{
            if link.start == start_node{
                link.dst
            }else if link.dst == start_node {
                link.start
            }else{
                panic!("start node not in link: 
                        given : {},
                        have: {} -> {}", start_node, link.start,link.dst);
            }
        }

        pub fn print_graph(&self) {
            println!("GRAPH:========================");
            let mut node_index = 0;
            for node in self.nodes.iter(){
                println!("name: {}", node.op_label);
                let mut index = 0;
                for port in node.ports.iter(){
                    if let Some(link) = port{
                        let link = &self.links[*link];
                        let dst_node  = self.get_node_linked_to(node_index, &link);
                        let dst_node = &self.nodes[dst_node];
                        print!("port: {}, ",index);
                        print!("dst: {}, ", self.operations.find(dst_node.op_label).unwrap().label);
                        println!("dst port: {}, ",link.dst_port);
                        println!("============================");
                    }
                    index+=1;
                }
                node_index+=1;
            }

            self.operations.print_rules();
        }

        pub fn attach(&mut self, op_name: &'a str) {
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
                            let res_free_port = self.nodes[rn].free_port().unwrap();
                            let new_link_index = self.links.len();

                            self.add_link(rn, new_node_index, res_free_port, new_node_free_port);

                            let res_node = &mut self.nodes[rn];
                            res_node.ports[res_free_port] = Some(new_link_index);
                            self.nodes[new_node_index].ports[new_node_free_port] = 
                                Some(new_link_index);

                            if res_free_port == 0{
                                println!("relinking result to : {}", 
                                    self.nodes.get(new_node_index).unwrap().op_label);
                                self.result = Some(new_node_index);
                            }
                        },
                    }
                },
            }
        }

        fn find_rule_to_apply(&self) -> Option<Box<[&'a RuleInfo]>>{
            let mut rule_to_apply = Vec::new();
            let mut index =0;
            for node in &self.nodes{
                println!("compute node {}", node.op_label);
                match &node.ports[node.main_port]{
                    None =>continue,
                    Some(link) =>{
                        let link = &self.links[*link];
                        let dst_node_index = self.get_node_linked_to(index, &link);
                        let dst_node = &self.nodes[dst_node_index];
                        println!("found link un main port of node : {} ,on port: {}, to: {}",
                            node.op_label, node.main_port, dst_node.op_label);
                        if let Some(back_link) = &dst_node.ports[dst_node.main_port] {
                            let back_link = &self.links[*back_link];
                            let back_ref_main_node = 
                                self.get_node_linked_to(dst_node_index, back_link);

                            if back_ref_main_node == index{
                                let port_op_conf ={
                                    let mut res = Vec::new();
                                    for port in node.ports.iter(){
                                        match port{
                                            None => res.push(None),
                                            Some(link) => {
                                                let link = &self.links[*link];
                                                let dst_node = 
                                                    self.get_node_linked_to(index, link);
                                                let dst_node = &self.nodes[dst_node];
                                                res.push(Some(dst_node.op_label));
                                            },
                                        }
                                    };
                                    res.into_boxed_slice()
                                };
                                if let Some(rule) = 
                                    self.operations.find_applicable_rule(
                                        node.op_label,
                                        &port_op_conf){
                                        println!("found computational step with rule: ");
                                        rule_to_apply.push(rule);
                                }
                            }
                        }
                    },
                };
                index+=1;
            }

            match rule_to_apply.len(){
                0 => None,
                _ => Some(rule_to_apply.into_boxed_slice())
            }
        }


        pub fn compute(&'a mut self){
            let rule_to_apply = self.find_rule_to_apply();
            match rule_to_apply{
                None => {
                },
                Some(rules) => {
                    for rule in rules.iter(){
                    }
                },
            }
        }
    }
}
