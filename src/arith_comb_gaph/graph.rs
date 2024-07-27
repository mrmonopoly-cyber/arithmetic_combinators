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

        pub fn print_rules(& self){
            self.operations.read().unwrap().print_rules();
        }

        pub fn print_graph(&self) {
            println!("GRAPH:(out: {})========================",self.result.read().unwrap().unwrap());
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
                        let new_node_index = {
                            let mut nodes = self.nodes.write().unwrap();
                            nodes.push(new_node);
                            nodes.len()-1
                        };
                        self.add_link_v2(node_index, new_node_index , index, 0);
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

            let mut links = links.write().unwrap();

            let links_start = links.len();
            for link_pattern in rule.subs.int_links{
                let start_node_index = start_position_new_nodes + link_pattern.start;
                let start_node_port = link_pattern.start_port;
                let dst_node_index = start_position_new_nodes + link_pattern.dst;
                let dst_node_port = link_pattern.end_port;
                let links_start = links.len();
                let link = Link { 
                    start: start_node_index, 
                    dst: dst_node_index,
                    start_port: start_node_port, 
                    dst_port: dst_node_port
                };

                links.push(link);
                {
                    let mut nodes = nodes.write().unwrap();
                    nodes[start_node_index].ports[start_node_port] = 
                        Some(links_start);
                    nodes[dst_node_index].ports[dst_node_port] =
                        Some(links_start);
                }
            };
            links_start
        }

        fn extract_aux_ext_port(node: &Node) -> Vec<Option<usize>> {
            let mut res = Vec::new();
            let arity = node.ports.len();
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

        fn link_old_nodes(
            nodes: &Arc<RwLock<Vec<Node<'a>>>>,
            links: &Arc<RwLock<Vec<Link>>>,
            start_position_new_nodes: usize,
            rule: &mut RuleInfo,
            main_node_index: usize,
            aux_node_index: usize,
            )
        {
            let aux_arity = 
            {
                let nodes_w = nodes.read().unwrap();
                let aux_node = &nodes_w[aux_node_index];
                aux_node.ports.len()
            };

            let ext_ports = {
                let nodes_w = nodes.read().unwrap();
                let main_node = &nodes_w[main_node_index];
                let aux_node = &nodes_w[aux_node_index];

                let mut ext_ports = Self::extract_aux_ext_port(main_node);
                let mut ext_aux_port_aux = Self::extract_aux_ext_port(aux_node);
                ext_ports.append(&mut ext_aux_port_aux);
                if aux_arity > 1{
                    match aux_node.ports[0]{
                        None => ext_ports.push(None),
                        Some(i) =>{
                            ext_ports.push(Some(i));
                        },
                    }
                }
                match main_node.ports[0]{
                    None => ext_ports.push(None),
                    Some(i) =>{
                        ext_ports.push(Some(i));
                    },
                }
                ext_ports
            };

            if let Some(ext_links) = rule.subs.ext_links{
                for link_index in ext_links{
                    let (link_0,link_1) = link_index;
                    let link_0 = ext_ports[*link_0];
                    let link_1 = ext_ports[*link_1];

                    match (link_0,link_1){
                        (None,Some(l)) | (Some(l),None) => {
                            let links = links.read().unwrap();
                            let link_0 = &links[l];
                            if  link_0.start == main_node_index || 
                                link_0.start == aux_node_index
                            {
                                let nodes =&mut nodes.write().unwrap();
                                nodes[link_0.dst].ports[link_0.dst_port] = None;
                                rule.subs.result_node = link_0.dst;

                            }else if    link_0.dst == main_node_index || 
                                link_0.dst == aux_node_index
                            {
                                let nodes =&mut nodes.write().unwrap();
                                nodes[link_0.start].ports[link_0.start_port] = None;
                                rule.subs.result_node = link_0.start;
                            }
                        },
                        (Some(l_1),Some(l_2)) =>{
                            fn find_to_change(link: &Link, main: usize, aux: usize) -> bool{
                                if link.start == main || link.start == aux{
                                    true
                                }else if link.dst == main || link.dst == aux{
                                    false
                                }else{
                                    panic!("main/aux node not found in link:
                                        start: {} -> dst: {},
                                        main: {},
                                        aux: {}",
                                        link.start,link.dst,
                                        main,
                                        aux);
                                }
                            }

                            let mut links_w = links.write().unwrap();
                            let mut nodes_w = nodes.write().unwrap();

                            let new_link_index = links_w.len();
                            let l_2 = links_w[l_2].clone();
                            let mut l_1_w = links_w[l_1].clone();

                            let l_1_change = find_to_change(&l_1_w, main_node_index, aux_node_index);
                            let l_2_change = find_to_change(&l_2, main_node_index, aux_node_index);

                            match (l_1_change,l_2_change){
                                (true,true) => {
                                    l_1_w.start = l_2.dst;
                                    l_1_w.start_port = l_2.dst_port;
                                },
                                (true,false) =>{
                                    l_1_w.start = l_2.start;
                                    l_1_w.start_port = l_2.start_port;
                                },
                                (false,true) => {
                                    l_1_w.dst = l_2.dst;
                                    l_1_w.dst_port= l_2.dst_port;
                                },
                                (false,false) => {
                                    l_1_w.dst = l_2.start;
                                    l_1_w.dst_port= l_2.start_port;
                                },
                            }
                            nodes_w[l_1_w.start].ports[l_1_w.start_port] = Some(new_link_index);
                            nodes_w[l_1_w.dst].ports[l_1_w.dst_port] = Some(new_link_index);
                            links_w.push(l_1_w);
                        },
                        _ => {},
                    }
                }
            }

            if let None = rule.subs.free_ports{
                return;
            }


            let mut ext_port_index = 0;
            for link_index in ext_ports.iter(){
                if let Some(link_index) = link_index{
                    let mut links = links.write().unwrap();
                    let link = &mut links[*link_index];
                    let rule = &rule.subs.free_ports.unwrap()[ext_port_index];
                    let new_node_index = start_position_new_nodes + rule.node;

                    if link.start == main_node_index || link.start == aux_node_index {
                        link.start = new_node_index;
                        link.start_port = rule.port;
                    }

                    if link.dst == main_node_index || link.dst == aux_node_index {
                        link.dst = new_node_index;
                        link.dst_port = rule.port;
                    }
                    {
                        let mut nodes = nodes.write().unwrap();
                        nodes[new_node_index].ports[rule.port]= Some(*link_index);
                    }

                    if link.start == link.dst{
                        let mut nodes = nodes.write().unwrap();
                        nodes[new_node_index].ports[link.start_port] = None;
                        nodes[new_node_index].ports[link.dst_port] = None;
                    }
                }
                ext_port_index+=1;
            }
        }

        fn delete_disable_nodes(
            nodes: &Arc<RwLock<Vec<Node<'a>>>>,
            node_index : usize,
            aux_node_index: Option<usize>,
            ){

            if let Some(aux_node_index) = aux_node_index{
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


        fn update_result_index(
            result: &Arc<RwLock<Option<usize>>>,
            old_main_index : usize,
            index_start_new_nodes: usize,
            rule: &RuleInfo,
            ){
            let mut _result = *result.read().unwrap();
            let new_res_node_index = {
                match rule.subs.free_ports{
                    None => {
                        rule.subs.result_node
                    },
                    Some(_) => {
                        index_start_new_nodes + rule.subs.result_node
                    },
                }
            };

            match _result{
                None =>{
                    result.write().unwrap().replace(new_res_node_index);
                },
                Some(m) => {
                    if m == old_main_index {
                        result.write().unwrap().replace(new_res_node_index);
                    }
                },

            }
        }

        pub fn get_result(& mut self) -> Option<i32>{
            let mut res = None;
            if let Some(r) = *self.result.read().unwrap(){
                let mut value = 0;
                let mut next = Some(r);
                while let Some(node_index) = next{
                    let node = &self.nodes.read().unwrap()[node_index];
                    match node.op_label{
                        "ZERO" => {
                            res = Some(value);
                            break;
                        },
                        "POS" => {
                            value+=1;
                            res = Some(value);
                        },
                        "NEG" => {
                            value-=1;
                            res = Some(value);
                        },
                        _ => panic!("invalid reduction"),
                    }

                    match node.ports[0] {
                        None => next = None,
                        Some(n) =>{
                            let link = &self.links.read().unwrap()[n];
                            if link.start == node_index{
                                next = Some(link.dst);
                            }else if link.dst == node_index{
                                next = Some(link.start);
                            }else{
                                panic!("invalid link node: {}", node_index);
                            }
                        },
                    }
                }
            }

            res
        }

        pub fn compute(&'static mut self){

            fn linked_to_port<'a> (
                nodes: &Arc<RwLock<Vec<Node<'a>>>>, 
                links: &Arc<RwLock<Vec<Link>>>,
                node_index : usize,
                port: usize) -> Option<usize>{
                let links = links.write().unwrap();
                let link ={
                    let nodes = nodes.read().unwrap();
                    let main_node = &nodes[node_index];
                    let link_to_aux = main_node.ports[port];
                    &links[link_to_aux.unwrap()]
                };
                if link.start == node_index{
                    Some(link.dst)
                }else if link.dst == node_index {
                    Some(link.start)
                }else{
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
                                let (mut rule,node_index) = rule;
                                let port = {
                                    let nodes = nodes.read().unwrap();
                                    nodes[node_index].main_port
                                };
                                let aux_node_index = linked_to_port(&nodes,&links, node_index,port);
                                let start_position_new_nodes = 
                                    Graph::adding_new_nodes(&operations, &nodes, &rule);

                                Graph::add_new_links(&nodes, &links, &rule, start_position_new_nodes);
                                
                                Graph::link_old_nodes(&nodes, &links,start_position_new_nodes,
                                    & mut rule, node_index, 
                                    aux_node_index.unwrap());

                                Graph::delete_disable_nodes(&nodes, node_index, aux_node_index);

                                Graph::update_result_index(
                                    &result, 
                                    node_index,
                                    start_position_new_nodes,
                                    &rule);
                            });
                            handler.push(handle);
                        }
                    },
                }
                for handle in handler{
                    handle.join().unwrap();
                }
            }
        }

        pub fn clear(&mut self){
            self.nodes.write().unwrap().clear();
            self.links.write().unwrap().clear();
            *self.result.write().unwrap() = None;
        }
    }
}
