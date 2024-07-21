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

    impl Link{
        pub fn connected_to(&self, endpoint : usize) -> usize{
            if self.start == endpoint{
                self.dst
            }else if self.dst == endpoint{
                self.start
            }else{
                panic!("endpoint not found:\n
                    given : {}\n
                    found: {} -> {}",endpoint,self.start,self.dst);
            }
        }

        pub fn print_link(&self){
            println!("({},{}) -> ({},{})",
            self.start,self.start_port,
            self.dst,self.dst_port);
        }
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

        fn add_link_v2(&mut self, start_node_index: usize, dst_node_index: usize,start_port :usize, dst_node_port: usize){
            let mut links = self.links.write().unwrap();
            let mut nodes = self.nodes.write().unwrap();

            let new_link_index = links.len();
            let new_link = Link{
                    start: start_node_index,
                    dst: dst_node_index,
                    start_port: start_port,
                    dst_port: dst_node_port,
            };
            links.push(new_link);

            {
                let start_node = &mut nodes[start_node_index];
                start_node.ports[start_port] = Some(new_link_index);
            }
            {
                let dst_node = &mut nodes[dst_node_index];
                dst_node.ports[dst_node_port] = Some(new_link_index);
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
                link.print_link();
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
                    print!("port {}: ",index);
                    match port{
                        None =>{
                            println!("EMPTY");
                        },
                        Some(link_index) => {
                            let link = &links[*link_index];
                            let dst_node_index  = self.get_node_linked_to(node_index, &link);
                            let dst_node = &nodes[dst_node_index];
                            print!("dst_index_node: {}, dst: {}, link_index: {}, ",
                                dst_node_index,
                                operations.find(dst_node.op_label).unwrap().label,
                                *link_index);
                            link.print_link();
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

        fn attach_v3(&mut self,
            node_index : usize, 
            parent_port: Option<usize>, 
            op_name: &'a str) -> bool {

            let ports = {
                let nodes = self.nodes.read().unwrap();
                let curr_node = &nodes[node_index];
                curr_node.ports.clone()
            };

            let mut index = ports.len()-1;
            for port in ports.iter().rev(){
                match port{
                    None =>{
                        let new_node= {
                            let ops = self.operations.read().unwrap();
                            Node::new(ops.find(op_name).unwrap())
                        };
                        let new_node_main_port = new_node.main_port;
                        let new_node_index = {
                            let mut nodes = self.nodes.write().unwrap();
                            nodes.push(new_node);
                            nodes.len()-1
                        };
                        self.add_link_v2(node_index, new_node_index , index, new_node_main_port);
                        return true;
                    },
                    Some(l_i) => {
                        let other_node_index = {
                            let links = self.links.read().unwrap();
                            let link = &links[*l_i];
                            link.connected_to(node_index)
                        };
                        let mut try_insert = false;
                        match parent_port {
                            None => try_insert =
                                self.attach_v3(other_node_index, Some(node_index), op_name),
                            Some(parent) =>{
                                if parent != other_node_index{
                                    try_insert =
                                        self.attach_v3(other_node_index, Some(node_index), op_name);
                                }
                            },
                        }

                        if try_insert{
                            return true;
                        }
                    },
                };
                if index > 0{
                    index-=1;
                }
            }

            false
        }

        pub fn attach(&mut self, op_name: &'a str) -> bool{
            let result = *self.result.write().unwrap();
            match result{
                None => {
                    let nodes = & mut self.nodes.write().unwrap();
                    let operations = self.operations.read().unwrap();
                    let new_node = Node::new(operations.find(op_name).unwrap());
                    *self.result.write().unwrap() = Some(0);
                    nodes.push(new_node);
                    true
                },
                Some(r) => {
                    self.attach_v3(r, None, op_name)
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
            start_position_new_nodes: usize,
            rule: &RuleInfo,
            main_node_index: usize,
            aux_node_index: usize,
            )
        {
            fn extract_aux_ext_port(node: &Node) -> Vec<Option<usize>> {
                let mut res = Vec::new();
                let arity = node.ports.len();
                println!("Node arity: {}",arity);
                if arity > 1{
                    let mut index = arity-1;
                    while index>1 {
                        match node.ports[index]{
                            None => res.push(None),
                            Some(i) => {
                                res.push(Some(i))
                            }
                        }
                        index-=1;
                    }
                }
                res
            }

            let mut nodes_w = nodes.write().unwrap();
            let mut links_w = links.write().unwrap();

            println!("main node index: {}",main_node_index);
            println!("aux node index: {}",aux_node_index);

            let main_node = &nodes_w[main_node_index];
            let main_arity = main_node.ports.len();
            let aux_node = &nodes_w[aux_node_index];
            let aux_arity = aux_node.ports.len();

            println!("main node label: {}",main_node.op_label);
            println!("aux node label: {}",aux_node.op_label);

            let mut ext_ports = extract_aux_ext_port(main_node);
            let mut ext_aux_port_aux = extract_aux_ext_port(aux_node);
            ext_ports.append(&mut ext_aux_port_aux);
            if aux_arity > 1{
                match aux_node.ports[0]{
                    None => ext_ports.push(None),
                    Some(i) =>{
                        ext_ports.push(Some(i));
                    },
                }
            }
            if main_arity > 1{
                match main_node.ports[0]{
                    None => ext_ports.push(None),
                    Some(i) =>{
                        ext_ports.push(Some(i));
                    },
                }
            }

            print!("ext ports: link inde: ");
            for p in &ext_ports{
                match p {
                    None => print!("None\t"),
                    Some(p) => print!("{}\t",p),
                }
            }
            println!("");


            let mut ext_port_index = 0;
            for link_index in ext_ports.iter(){
                print!("ext port: {}\t", ext_port_index);
                if let Some(link) = link_index{
                    let link = &links_w[*link];
                    print!("link info: ({},{}) -> ({},{})", 
                        link.start,link.start,link.dst,link.dst_port);
                }
                println!();
                ext_port_index+=1;
            }

            let mut ext_port_index = 0;
            for link_index in ext_ports.iter(){
                if let Some(link_index) = link_index{
                    let link = &mut links_w[*link_index];
                    let rule = &rule.subs.free_ports[ext_port_index];
                    let new_node_index = start_position_new_nodes + rule.node;

                    println!("link: {} -> {}", link.start,link.dst);
                    println!("main node : {}", main_node_index);
                    println!("aux node : {}", aux_node_index);

                    if link.start == main_node_index || link.start == aux_node_index {
                        link.start = new_node_index;
                        link.start_port = rule.port;
                    }

                    if link.dst == main_node_index || link.dst == aux_node_index {
                        link.dst = new_node_index;
                        link.dst_port = rule.port;
                    }
                    nodes_w[new_node_index].ports[rule.port]= Some(*link_index);
                }
                ext_port_index+=1;
            }
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

                                Graph::link_old_nodes(&nodes, &links,start_position_new_nodes,
                                    &rule, node_index, 
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
