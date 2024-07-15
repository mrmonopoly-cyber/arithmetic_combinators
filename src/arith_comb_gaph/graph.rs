pub mod graph{
    use core::panic;
    use std::{sync::{Arc, RwLock}, usize, vec};
    use std::thread;

    use crate::arith_comb_gaph::{operation::operations::Operation, operation_pool};

    use super::super::operation_pool::operation_pool::*;

    #[derive(Debug,Clone)]
    pub struct Graph<'a>{
        operations: Arc<RwLock<OpPool<'a>>>,
        nodes: Arc<RwLock<Vec<Node<'a>>>>,
        links: Arc<RwLock<Vec<Link>>>,
        result: Option<usize>,
    }

    #[derive(Debug)]
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
    
        fn extract_aux_port(&self) ->Vec<Option<usize>>{
            let arity = self.ports.len();
            match arity{
                0 => panic!("invalid arity of node: {}",0),
                1 => {
                    vec![self.ports[0]]
                },
                _ => {
                    let mut res = Vec::new();
                    let main_port = 1;
                    let mut index = 0;
                    for port in self.ports.iter(){
                        if index != main_port{
                            match port {
                                None => res.push(None),
                                Some(i) => res.push(Some(*i)),
                            }
                        }
                        index+=1;
                    }
                    res
                },
            }
        }
        
    }
    
    fn add_link(links: &mut std::sync::RwLockWriteGuard<Vec<Link>>, start_node: usize, dst_node: usize,start_port :usize, dst_node_port: usize){
        links.push(
            Link{
                start: start_node,
                dst: dst_node,
                start_port: start_port,
                dst_port: dst_node_port,
            }
        );
    }
    impl<'a> Graph<'a> {
        pub fn new(ops: OpPool<'a>) -> Self{
            Self{
                operations: Arc::new(RwLock::new(ops)),
                nodes: Arc::new(RwLock::new(Vec::new())),
                links: Arc::new(RwLock::new(Vec::new())),
                result: None,
            }
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
            let links = & mut self.links.read().unwrap();

            let mut node_index = 0;
            let nodes = self.nodes.read().unwrap();
            let operations = self.operations.read().unwrap();
            for node in nodes.iter(){
                println!("name: {}", node.op_label);
                let mut index = 0;
                for port in node.ports.iter(){
                    if let Some(link) = port{
                        let link = &links[*link];
                        let dst_node  = self.get_node_linked_to(node_index, &link);
                        let dst_node = &nodes[dst_node];
                        print!("port: {}, ",index);
                        print!("dst: {}, ", operations.find(dst_node.op_label).unwrap().label);
                        println!("dst port: {}, ",link.dst_port);
                        println!("============================");
                    }
                    index+=1;
                }
                node_index+=1;
            }

            operations.print_rules();
        }

        pub fn attach(&mut self, op_name: &'a str) {
            let nodes = & mut self.nodes.write().unwrap();
            let operations = self.operations.read().unwrap();
            let links = &mut self.links.write().unwrap();

            let op = operations.find(op_name);
            match op{
                None => (),
                Some(op) =>{
                    let new_node = Node::new(op);
                    let new_node_free_port = new_node.free_port().unwrap();
                    let new_node_index = nodes.len();
                    nodes.push(new_node);

                    match self.result {
                        None =>{
                            return self.result = Some(0);
                        },
                        Some(rn) =>{
                            let res_free_port = nodes[rn].free_port().unwrap();
                            let new_link_index = links.len();

                            add_link(links, rn, new_node_index, res_free_port, new_node_free_port);

                            let res_node = &mut nodes[rn];
                            res_node.ports[res_free_port] = Some(new_link_index);
                            nodes[new_node_index].ports[new_node_free_port] = 
                                Some(new_link_index);

                            if res_free_port == 0{
                                println!("relinking result to : {}", 
                                    nodes.get(new_node_index).unwrap().op_label);
                                self.result = Some(new_node_index);
                            }
                        },
                    }
                },
            }
        }

        fn find_rule_to_apply(&self) -> Option<Box<[(RuleInfo,usize)]>>{
            let nodes = self.nodes.read().unwrap();
            let operations = self.operations.read().unwrap();
            let links = self.links.read().unwrap();

            let mut rule_to_apply : Vec<(RuleInfo,usize)>= Vec::new();
            let mut index =0;
            for node in nodes.iter(){
                println!("compute node {}", node.op_label);
                match &node.ports[node.main_port]{
                    None =>continue,
                    Some(link) =>{
                        let link = &links[*link];
                        let dst_node_index = self.get_node_linked_to(index, link);
                        let dst_node = &nodes[dst_node_index];
                        println!("found link un main port of node : {} ,on port: {}, to: {}",
                            node.op_label, node.main_port, dst_node.op_label);
                        if let Some(back_link) = &dst_node.ports[dst_node.main_port] {
                            let back_link = &links[*back_link];
                            let back_ref_main_node = 
                                self.get_node_linked_to(dst_node_index, back_link);

                            if back_ref_main_node == index{
                                let port_op_conf ={
                                    let mut res = Vec::new();
                                    for port in node.ports.iter(){
                                        match port{
                                            None => res.push(None),
                                            Some(link) => {
                                                let link = &links[*link];
                                                let dst_node = 
                                                    self.get_node_linked_to(index, link);
                                                let dst_node = &nodes[dst_node];
                                                res.push(Some(dst_node.op_label));
                                            },
                                        }
                                    };
                                    res.into_boxed_slice()
                                };
                                let rule = operation_pool::operation_pool::find_applicable_rule(
                                        &operations,
                                        node.op_label,
                                        port_op_conf);

                                if let Some(rule) = rule{
                                        println!("found computational step with rule: ");
                                        rule_to_apply.push((rule,index));
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

        pub fn compute(&'static mut self){
            let rule_to_apply = self.find_rule_to_apply();
            let mut handler = vec![];
            let arc_self = Arc::new(&self);

            match rule_to_apply{
                None => {
                },
                Some(rules) => {
                    for rule in rules.iter(){
                        let rule = rule.clone();
                        let nodes = Arc::clone(&self.nodes);
                        let operations = Arc::clone(&self.operations);
                        let links = Arc::clone(&self.links);
                        let arc_self = Arc::clone(&arc_self);

                        let handle = thread::spawn(move || {
                            let (rule,node_index) = rule;
                            let operations = operations.read().unwrap();

                            println!("applying substitution on node: {}",rule.main_node_label);

                            println!("adding the new nodes to the graph");
                            let mut start_position_new_nodes = None;
                            {
                                let mut nodes = nodes.write().unwrap();
                                start_position_new_nodes = Some(nodes.len());
                                for op_name in rule.subs.new_nodes_labels{
                                    let op = operations.find(op_name).unwrap();
                                    nodes.push(Node::new(op));
                                }
                            }
                            let start_position_new_nodes = start_position_new_nodes.unwrap();

                            println!("adding the new links to the graph");
                            let mut start_position_new_links = None;
                            {
                                let mut links = links.write().unwrap();
                                let mut nodes = nodes.write().unwrap();

                                let links_start = links.len();
                                let mut cursor =0;
                                for link_pattern in rule.subs.int_links{
                                    let start_node_index = start_position_new_nodes + link_pattern.start;
                                    let start_node_port = link_pattern.start_port;
                                    let dst_node_index = start_position_new_nodes + link_pattern.dst;
                                    let dst_node_port = link_pattern.end_port;

                                    links.push(Link { 
                                        start: start_node_index, 
                                        dst: dst_node_index,
                                        start_port: start_node_port, 
                                        dst_port: dst_node_port
                                    });
                                    nodes[start_node_index].ports[start_node_port] = 
                                        Some(links_start + cursor);
                                    nodes[dst_node_index].ports[dst_node_port] =
                                        Some(links_start + cursor);
                                    cursor+=1;
                                };
                                start_position_new_links = Some(links_start);
                            }
                            let start_position_new_links = start_position_new_links.unwrap();

                            println!("linking new nodes to old graph");
                            {
                                let nodes = nodes.write().unwrap();
                                let links = links.write().unwrap();

                                let old_main_node = &nodes[node_index];
                                let old_aux_node = &nodes[old_main_node.main_port];
                                let mut old_main_node_aux_port = old_main_node.extract_aux_port();
                                let mut old_aux_node_aux_port = old_aux_node.extract_aux_port();
                                old_main_node_aux_port.append(&mut old_aux_node_aux_port);

                                let mut port_index =0;
                                for free_port in rule.subs.free_ports{
                                    let link = &links[port_index];
                                    port_index+=1;
                                }
                            }
                        });
                        handler.push(handle);
                    }
                },
            }
            for handle in handler{
                handle.join().unwrap();
            }
            self.print_graph();
        }
    }
}
