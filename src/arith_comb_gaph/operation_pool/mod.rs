pub mod operation_pool{
    use super::super::operation::operations::*;

    #[derive(Debug)]
    pub struct OpPool<'a> {
        ops: Box<[Operation<'a>]>,
        rules: Vec<RuleInfo<'a>>,
    }

    #[derive(Debug,PartialEq,Clone)]
    pub struct RuleInfo<'a> {
        pub main_node_label: &'a str,
        pub conf: Box<[Option<usize>]>,
        pub subs: SubPattern<'a>,
    }

    #[derive(Debug,PartialEq)]
    struct SubIntLink{
        start: usize,
        dst: usize,
        start_port: usize,
        end_port: usize,
    }

    #[derive(Debug,PartialEq)]
    struct SubFreePort{
        node: usize,
        port: usize,
    }

    #[derive(Debug,PartialEq,Clone)]
    pub struct SubPattern<'a> {
        new_nodes_labels: &'a[&'a str],
        links: &'a[&'a SubIntLink],
        int_links: &'a [&'a SubIntLink],
        ext_links: &'a [(usize,usize)],
        free_ports: &'a[&'a SubFreePort],
    }

    impl<'a> SubPattern<'a> {
        pub fn new() -> Self {
            Self{
            }
        }
    }



    impl<'a> OpPool<'a> {
        pub fn new(ops: Box<[Operation<'a>]>) -> Self {
            Self{
                ops: ops,
                rules: Vec::new(),
            }
        }

        pub fn find_applicable_rule
            (&self, 
             main_node_label : &'a str,
             main_port_label : &Box<[Option<&str>]>,) -> Option<&RuleInfo>  {
                for rule in self.rules.iter(){
                    if rule.main_node_label == main_node_label{
                        let rule_conf_port = {
                            let mut res = Vec::new();
                            for port in rule.conf.iter(){
                                match port{
                                    None => res.push(None),
                                    Some(port) =>{
                                        let port_label = self.ops[*port].label;
                                        res.push(Some(port_label));
                                    },
                                };
                            };
                            res.into_boxed_slice()
                        };
                        if rule_conf_port.eq(main_port_label) {
                            return Some(rule)
                        }
                    }
                }
                None
            }

        pub fn find(&self, name:&'a str) ->Option<&Operation<'a>>{
            let mut res =None;
            for op in self.ops.iter(){
                if op.label == name{
                    res = Some(op)
                }
            }
            res
        }

        pub fn add_rule(&mut self, 
            main_node_label : &'a str,
            new_rules:(&[Option<&'a str>],SubPattern<'a>)){
            let (confs,sub) = new_rules;

            let conf_rule = {
                let mut res = Vec::new();

                for conf in confs{
                    match *conf {
                        None => res.push(None),
                        Some(op_r) => {
                            let mut index = 0;
                            for op in self.ops.iter(){
                                if op.label == op_r{
                                    res.push(Some(index));
                                    break;
                                }
                                index+=1;
                            }
                            // panic!("op not found for label : {}", op_r)
                        },
                    }
                }

                res.into_boxed_slice()
            };

            self.rules.push(
                RuleInfo { 
                    main_node_label: main_node_label,
                    conf: conf_rule, 
                    subs: sub 
                }
            );

        }


        pub fn print_rules(&self){
            println!("RULES===============================");
            let mut rule_index = 0;
            for rule in self.rules.iter() {
                let mut port_index =0;
                println!("rule: {}, ",rule_index);
                println!("main op: {}, ",rule.main_node_label);
                for port in rule.conf.iter(){
                    print!("port: {}, ",port_index);
                    print!("value: ");
                    match port {
                        None => println!("None"),
                        Some(op_index) =>{
                            println!("{}",self.ops[*op_index].label);
                        },
                    }
                    port_index+=1;
                }
                rule_index+=1;
                println!("+++++++++++++++++++++++++++++");
            }
        }
    }
}
