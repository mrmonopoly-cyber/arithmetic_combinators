pub mod operation_pool{
    use super::super::operation::operations::*;

    #[derive(Debug)]
    pub struct OpPool<'a> {
        ops: Box<[Operation<'a>]>,
        rules: Vec<Rule>,
    }

    #[derive(Debug,PartialEq,Clone)]
    pub struct Rule {
        main_active_op_label: usize,
        other_active_op_label: usize,
        possibilities: Option<Box<[RuleInfo]>>,
    }

    #[derive(Debug,PartialEq,Clone)]
    struct RuleInfo {
        conf: Box<[Option<usize>]>,
        subs: usize,
    }


    impl<'a> OpPool<'a> {
        pub fn new(ops: Box<[Operation<'a>]>) -> Self {
            Self{
                ops: ops,
                rules: Vec::new(),
            }
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

        pub fn find_index(&self, name:&'a str) ->Option<usize>{
            let mut res =None;
            let mut index = 0;
            for op in self.ops.iter(){
                if op.label == name{
                    res = Some(index);
                    break
                }
                index+=1;
            }
            res
        }

        fn generate_conf_port(&self, port_conf: &'a[Option<&'a str>] ) -> RuleInfo{
            let mut vec_conf = Vec::new();
            for port in port_conf{
                match *port {
                    None => vec_conf.push(None),
                    Some(label) =>{
                        vec_conf.push(self.find_index(label))
                    }
                }
            }
            RuleInfo{
                conf: vec_conf.into_boxed_slice(),
                subs: 0,
            }
        }

        pub fn add_rule(&mut self,
                        main_comb: &'a str, 
                        aux_comb: &'a str,
                        new_rules: Option<Box<[&'a [Option<&'a str>]]>>){
            let main_comb = self.find_index(main_comb);
            let aux_comb = self.find_index(aux_comb);

            match (main_comb,aux_comb){
                (Some(main_comb),Some(aux_comb)) =>{
                    let mut pool_rule = Rule { 
                        main_active_op_label: main_comb, 
                        other_active_op_label: aux_comb, 
                        possibilities: None };
                    match new_rules {
                        None => (),
                        Some(new_rules) =>{
                            let mut vec_rule_info = Vec::new();
                            for new_rule in new_rules.iter() {
                                vec_rule_info.push(self.generate_conf_port(*new_rule));
                            }
                            pool_rule.possibilities = Some(vec_rule_info.into_boxed_slice());
                        },
                    };
                    self.rules.push(pool_rule)
                },
                _ => (),
            }
        }

        fn print_single_port_conf(&self, conf: &Option<Box<[RuleInfo]>>) {
            println!("rule info:");
            match conf {
                None => println!("no rule for this operation"),
                Some(rules) =>{
                    let mut rule_index = 0;
                    for rule in rules.iter() {
                        let mut port_index =0;
                        let port_last = rule.conf.len()-1;
                        println!("rule: {}, ",rule_index);
                        for port in rule.conf.iter(){
                            print!("port: {}, ",port_last - port_index);
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
                },
            }
        }

        pub fn print_rules(&self){
            println!("RULES===============================");
            let op_labels = &self.ops;
            for rule in &self.rules{
                println!("main comb: {},", op_labels[rule.main_active_op_label].label);
                println!("aux comb: {},", op_labels[rule.other_active_op_label].label);
                self.print_single_port_conf(&rule.possibilities);
                println!("-------------------------------");
            }
        }
    }
}
