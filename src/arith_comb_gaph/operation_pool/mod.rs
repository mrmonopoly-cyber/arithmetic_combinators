pub mod operation_pool{
    use super::super::operation::operations::*;

    #[derive(Debug)]
    pub struct OpPool<'a> {
        ops: Box<[Operation<'a>]>,
        rules: Vec<Rule<'a>>,
    }

    #[derive(Debug,PartialEq,Clone)]
    pub struct Rule<'a> {
        main_active_op_label: usize,
        other_active_op_label: usize,
        possibilities: Option<Box<[RuleInfo<'a>]>>,
    }

    #[derive(Debug,PartialEq,Clone)]
    pub struct RuleInfo<'a> {
        pub conf: Box<[Option<usize>]>,
        pub subs: SubPattern<'a>,
    }

    #[derive(Debug,PartialEq,Clone)]
    pub struct SubPattern<'a> {
        new_op: Box<[&'a str]>,
        // index op dst, dst port
        ext_link: Box<[(usize,usize)]>, 
        //index start op, port start port
        //index end op, index end port
        int_link: Box<[(usize,usize,usize,usize)]>, 
    }

    impl<'a> SubPattern<'a> {
        pub fn new() -> Self {
            Self{
                ext_link: Box::new([]),
                new_op: Box::new([]),
                int_link: Box::new([]),
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

        pub fn find_applicable_rule(&self, 
            main_node_label: &str,
            aux_node_label: &str,
            main_port_label : &Box<[Option<& str>]>,
            aux_port_label: &Box<[Option<& str>]>) -> Option<&Rule>  {

            let mut rule_container = None;
            for rule in self.rules.iter(){
                let main_rule_node = &self.ops[rule.main_active_op_label];
                let auxi_rule_node = &self.ops[rule.other_active_op_label];
                if  main_rule_node.label == main_node_label &&
                    auxi_rule_node.label == aux_node_label {
                        rule_container = Some(rule);
                }
            };

            let mut res = None;
            if let Some(rule) = rule_container{
                if let Some(poss) = &rule.possibilities{
                    for sub in poss.iter(){
                    }
                }
            }
            res
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

        pub fn generate_conf_port(&self, port_conf: &'a[Option<&'a str>] ) -> RuleInfo<'a>{
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
                subs: SubPattern{
                    new_op: Box::new([]),
                    ext_link: Box::new([]),
                    int_link: Box::new([]),
                },
            }
        }

        pub fn add_rule(&mut self,
                        main_comb: &'a str, 
                        aux_comb: &'a str,
                        new_rules:Option<Box<[(&'a[Option<&'a str>],SubPattern<'a>)]>>){

            let new_rules = {
                match new_rules{
                    None => None,
                    Some(conf) =>{
                        let mut vec_rule = Vec::new();
                        for (conf,_subs) in conf.iter(){
                            vec_rule.push(self.generate_conf_port(conf));
                        }
                        Some(vec_rule.into_boxed_slice())
                    },
                }
            };
            let main_comb_i = self.find_index(main_comb);
            let aux_comb_i = self.find_index(aux_comb);
            match (main_comb_i,aux_comb_i){
                (Some(main_comb),Some(aux_comb)) =>{
                    let pool_rule = Rule{ 
                        main_active_op_label: main_comb, 
                        other_active_op_label: aux_comb, 
                        possibilities: new_rules};
                    self.rules.push(pool_rule)
                },
                _ => {
                    println!("not found op: main {}, aux: {}", main_comb,aux_comb);
                },
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

        pub fn print_rule(&self, rule :&Rule){
            println!("Rule of main: {}, to aux: {}",
                self.ops[rule.main_active_op_label].label,
                self.ops[rule.other_active_op_label].label);
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
