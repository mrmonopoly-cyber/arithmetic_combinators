pub mod operation_pool{
    use core::panic;

    use super::super::operation::operations::*;

    pub type PortConf = Option<Box<[Box<[Option<usize>]>]>>;
    #[derive(Debug)]
    pub struct OpPool<'a> {
        ops: Box<[Operation<'a>]>,
        rules: Vec<Rule>,
    }

    #[derive(Debug,PartialEq,Clone)]
    pub struct Rule {
        main_active_op_label: usize,
        other_active_op_label: usize,
        port_conf: PortConf,
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

        fn generate_conf_port(&self, port_conf: Option<Box<[Box<[&'a str]>]>>) -> PortConf {
            match port_conf {
                None => None,
                Some(port_conf) => {
                    let mut res = Vec::new();
                    for conf in port_conf.iter(){
                        let mut single_conf = Vec::<Option<usize>>::new();
                        for port in conf.iter(){
                            let op_pos = self.find_index(*port);
                            match op_pos{
                                None => panic!("invalid conf label: {}", *port),
                                _ => single_conf.push(op_pos),
                            };
                        }
                        res.push(single_conf.into_boxed_slice());
                    }
                    Some(res.into_boxed_slice())
                },
            }
        }

        pub fn add_rule(&mut self,
                        main_comb: &'a str, 
                        aux_comb: &'a str,
                        port_conf: Option<Box<[Box<[&'a str]>]>>){
            let main_comb = self.find_index(main_comb);
            let aux_comb = self.find_index(aux_comb);

            match (main_comb,aux_comb){
                (Some(main_comb),Some(aux_comb)) =>{
                    let new_rule = Rule{
                        port_conf: self.generate_conf_port(port_conf),
                        main_active_op_label: main_comb,
                        other_active_op_label: aux_comb,
                    };
                    self.rules.push(new_rule);
                },
                _ => (),
            }
        }
    }
}
