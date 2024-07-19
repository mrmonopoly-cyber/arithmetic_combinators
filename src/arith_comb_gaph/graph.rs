pub mod graph{
    use core::panic;
    use std::{sync::{Arc, RwLock}, vec};
    use std::thread;

    use crate::arith_comb_gaph::operation::operations::Operation;

    use super::super::operation_pool::operation_pool::*;

    #[derive(Debug,Clone)]
    pub struct Graph<'a>{
        operations: Arc<RwLock<OpPool<'a>>>,
        nodes: Arc<RwLock<Vec<Node<'a>>>>,
        links: Arc<RwLock<Vec<Link>>>,
        result: Arc<RwLock<Option<usize>>>,
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
                result: Arc::new(RwLock::new(None)),
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

        pub fn print_links(&self){
            println!("LINKS:========================");
            let links = self.links.read().unwrap();
            let mut index =0;
            for link in links.iter(){
                println!("link index: {}.",index);
                println!("node start/dst {} <-> {}.",link.start,link.dst);
                println!("port start/dst {} <-> {}.",link.start_port,link.dst_port);
                index+=1;
            }
        }

        pub fn print_graph(&self) {
            println!("GRAPH:========================");
            let links = & mut self.links.read().unwrap();

            let mut node_index = 0;
            let nodes = self.nodes.read().unwrap();
            let operations = self.operations.read().unwrap();
            for node in nodes.iter(){
                println!("index: {}, name: {}",node_index, node.op_label);
                let mut index = 0;
                for port in node.ports.iter(){
                    match port{
                        None =>{
                            println!("port {}: EMPTY",index);
                        },
                        Some(link_index) => {
                            let link = &links[*link_index];
                            let dst_node_index  = self.get_node_linked_to(node_index, &link);
                            let dst_node = &nodes[dst_node_index];
                            print!("dst_index_node: {}, dst: {}, link_index: {}, ",
                                dst_node_index,
                                operations.find(dst_node.op_label).unwrap().label,
                                *link_index);
                            print!("port: {}, ",index);
                            println!("-> port: {}, ",
                                if link.start == dst_node_index{
                                    link.start_port
                                }else if link.dst == dst_node_index{
                                    link.dst_port
                                }else{
                                    panic!("invalid node index:
                                        given: {},
                                        have : {} -> {}",
                                        dst_node_index, link.start, link.dst);

                                }
                            );
                        },
                    }
                    index+=1;
                }
                println!("============================");
                node_index+=1;
            }

            self.print_links();
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

                    let result = *self.result.read().unwrap();

                    match result{
                        None =>{
                            *self.result.write().unwrap() = Some(0);
                            return;
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
                                *self.result.write().unwrap() = Some(new_node_index);
                            }
                        },
                    }
                },
            }
        }

        fn find_node_rule(&self, node_index : usize) -> Option<(RuleInfo,usize)>{
            let mut res = Vec::new();
            let links = self.links.read().unwrap();
            let nodes = self.nodes.read().unwrap();
            let node = &nodes[node_index];

            for port in node.ports.iter(){
                if let Some(port) = port{
                    let link = &links[*port];
                    let node_linked_to = self.get_node_linked_to(node_index, link);
                    let node_op = nodes[node_linked_to].op_label;
                    res.push(Some(node_op));
                }else{
                    res.push(None);
                }
            }

            let operations = self.operations.read().unwrap();
            let rule = operations.find_applicable_rule(node.op_label, res.into_boxed_slice());
            match rule{
                None => None,
                Some(rule) => {
                    Some((rule,node_index))
                }
            }
        }

        fn find_rule_to_apply(&self) -> Option<Box<[(RuleInfo,usize)]>>{
            let nodes = self.nodes.read().unwrap();
            let links = self.links.read().unwrap();

            let mut res = Vec::new();
            for link in links.iter(){
                let start_node = &nodes[link.start];
                let dst_node = &nodes[link.dst];

                if  link.start_port == start_node.main_port && 
                    link.dst_port == dst_node.main_port {

                        let rule_to_apply = self.find_node_rule(link.start);
                        if let Some(rule) = rule_to_apply{
                            res.push(rule);
                        }
                        let rule_to_apply = self.find_node_rule(link.dst);
                        if let Some(rule) = rule_to_apply{
                            res.push(rule);
                        }
                }
            }
            
            if res.len() > 0 {
                Some(res.into_boxed_slice())
            }else{
                None
            }

        }

        fn adding_new_nodes( operations: &Arc<RwLock<OpPool<'a>>>, 
                nodes : &Arc<RwLock<Vec<Node<'a>>>>,
                rule: &RuleInfo<'a>)-> usize{

            println!("adding the new nodes to the graph");
            let operations = operations.read().unwrap();
            let mut nodes = nodes.write().unwrap();
            let res = nodes.len();

            for op_name in rule.subs.new_nodes_labels{
                let op = operations.find(op_name).unwrap();
                nodes.push(Node::new(op));
            }

            res
        }

        fn add_new_links(
                nodes: &Arc<RwLock<Vec<Node<'a>>>>,
                links: &Arc<RwLock<Vec<Link>>>,
                rule: &RuleInfo,
                start_position_new_nodes: usize
            ) -> usize{

            println!("adding the new links to the graph");
            let mut links = links.write().unwrap();
            let mut nodes = nodes.write().unwrap();

            let links_start = links.len();
            for link_pattern in rule.subs.int_links{
                let start_node_index = start_position_new_nodes + link_pattern.start;
                let start_node_port = link_pattern.start_port;
                let dst_node_index = start_position_new_nodes + link_pattern.dst;
                let dst_node_port = link_pattern.end_port;
                let links_start = links.len();

                links.push(Link { 
                    start: start_node_index, 
                    dst: dst_node_index,
                    start_port: start_node_port, 
                    dst_port: dst_node_port
                });
                println!("adding int link:");
                println!("start node index: {}", start_node_index);
                println!("start node port: {}", start_node_port);
                println!("dst node index: {}", dst_node_index);
                println!("dst node port: {}", dst_node_port);

                nodes[start_node_index].ports[start_node_port] = 
                    Some(links_start);
                nodes[dst_node_index].ports[dst_node_port] =
                    Some(links_start);
            };
            links_start
        }

        fn link_old_nodes(
            nodes: &Arc<RwLock<Vec<Node<'a>>>>,
            links: &Arc<RwLock<Vec<Link>>>,
            rule: &RuleInfo,
            main_node_index: usize,
            aux_node_index: usize,
            )
        {
            let mut nodes_w = nodes.write().unwrap();

            println!("main node index: {}",main_node_index);
            println!("aux node index: {}",aux_node_index);

            let main_node = &nodes_w[main_node_index];
            let aux_node = &nodes_w[aux_node_index];

            println!("main node label: {}",main_node.op_label);
            println!("aux node label: {}",aux_node.op_label);

        }

        fn delete_disable_nodes(
            nodes: &Arc<RwLock<Vec<Node<'a>>>>,
            node_index : usize,
            aux_node_index: Option<usize>,
            ){

            println!("disabling old nodes");
            if let Some(aux_node_index) = aux_node_index{
                println!("deleting old aux");
                let mut nodes = nodes.write().unwrap();
                let old_aux_node = &mut nodes[aux_node_index];
                old_aux_node.ports.fill(None);
            }
            {
                let mut nodes = nodes.write().unwrap();
                let old_main_node = &mut nodes[node_index];
                old_main_node.ports.fill(None);
            }
        }

        fn update_result_inde(
            result: &Arc<RwLock<Option<usize>>>,
            old_main_index : usize,
            new_res_node_index : usize,
            ){
            let mut _result = *result.write().unwrap();
            match _result{
                None =>{
                    println!("updating result index node to: {}", new_res_node_index);
                    _result = Some(new_res_node_index);
                },
                Some(m) => {
                    if m == old_main_index {
                        println!("updating result index node to: {}", new_res_node_index);
                        _result = Some(new_res_node_index);
                    }
                },

            }
        }

        pub fn compute(&'static mut self){

            fn linked_to_port<'a> (
                nodes: &Arc<RwLock<Vec<Node<'a>>>>, 
                links: &Arc<RwLock<Vec<Link>>>,
                node_index : usize,
                port: usize) -> Option<usize>{
                let nodes = nodes.write().unwrap();
                let links = links.write().unwrap();
                let main_node = &nodes[node_index];
                let link_to_aux = main_node.ports[port];
                let link = &links[link_to_aux.unwrap()];
                if link.start == node_index{
                    println!("found start");
                    Some(link.dst)
                }else if link.dst == node_index {
                    println!("found dst");
                    Some(link.start)
                }else{
                    println!("found none");
                    None
                }
            }

            loop{
                let rule_to_apply = self.find_rule_to_apply();
                let mut handler = vec![];
                match rule_to_apply{
                    None => break,
                    Some(rules) => {
                        for rule in rules.iter(){
                            let rule = rule.clone();
                            let nodes = Arc::clone(&self.nodes);
                            let operations = Arc::clone(&self.operations);
                            let links = Arc::clone(&self.links);
                            let result = Arc::clone(&self.result);

                            let handle = thread::spawn(move || {
                                let (rule,node_index) = rule;
                                let port = {
                                    let nodes = nodes.read().unwrap();
                                    nodes[node_index].main_port
                                };
                                println!("applying substitution on node: {}",rule.main_node_label);
                                let aux_node_index = linked_to_port(&nodes,&links, node_index,port);
                                let start_position_new_nodes = 
                                    Graph::adding_new_nodes(&operations, &nodes, &rule);

                                Graph::add_new_links(&nodes, &links, &rule, start_position_new_nodes);

                                Graph::link_old_nodes(&nodes, &links, &rule, node_index, 
                                    aux_node_index.unwrap());

                                Graph::delete_disable_nodes(&nodes, node_index, aux_node_index);

                                Graph::update_result_inde(&result, 
                                    node_index,
                                    start_position_new_nodes + rule.subs.result_node);
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
}
