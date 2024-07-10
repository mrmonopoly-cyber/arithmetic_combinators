pub mod basic_combinator{
    use crate::{combinator::GenericCombinator, graph::CombGraph};

    type PortType<'a> = Option<&'a BasicCombinator<'a>>;
    type AuxPortType<'a> = Option<&'a [PortType<'a>]>;
    type NodeLabel = u8;

    #[derive(Debug)]
    pub struct BasicCombinator<'a> {
        name: &'a str,
        label: NodeLabel,
        main_port: PortType<'a>,
        auxi_port: AuxPortType<'a>,
        rules: Option<&'a[CombRule<'a>]>,
    }

    #[derive(Debug)]
    pub struct CombRule<'a> {
        pub prim_port_label: NodeLabel,
        pub aux_port_labels: Option<&'a[Option<NodeLabel>]>,
        pub substitution: CombGraph<'a>,
    }

    impl <'a> BasicCombinator<'a> {
        pub fn new_basic_combinator(label: NodeLabel, 
                                    name: &'a str,
                                    aux_port: AuxPortType<'a>,
                                    rules: Option<&'a[CombRule<'a>]>) -> BasicCombinator<'a> {
            BasicCombinator { 
                name: name,
                label: label,
                main_port: None, 
                auxi_port: aux_port,
                rules: rules,
            }
        }

        fn can_apply(&self, main_port_comb: &BasicCombinator<'a>, rule: &CombRule<'a>) -> bool{
            if  main_port_comb.get_lable_id() == rule.prim_port_label {
                match (self.auxi_port, rule.aux_port_labels){
                    (None,None) => true,
                    (Some(s_aux_port),Some(r_aux_port)) => 
                    {
                        let s_aux_size = s_aux_port.len();
                        let r_aux_size = r_aux_port.len();

                        if s_aux_size == r_aux_size{
                            for i in 0..s_aux_size {
                                match (s_aux_port[i],r_aux_port[i]){
                                    (Some(_),None) | (None,Some(_)) => return false,
                                    (Some(s_lab),Some(r_lab)) =>{
                                        if s_lab.get_lable_id() != r_lab {
                                            return false;
                                        }
                                    },
                                    (None,None) => {},
                                }
                            }
                            return true
                        }
                        false
                    },
                    _ => false
                }
            }else{
                false
            }
        }
    }

    impl <'a> GenericCombinator for BasicCombinator<'a>{
        fn get_lable_id(&self) -> NodeLabel {
            self.label
        }

        fn get_lable_name(&self) -> &str {
            self.name
        }

        fn compute(&self) -> Option<CombGraph<'a>> {
            match (self.main_port,self.rules) {
                (Some(node),Some(rules)) => {
                    for rul in rules{
                        if self.can_apply(node, rul){
                            Some(rul.substitution);
                        }
                    }
                    None
                },
                _ => None
            }
        }
    }
}
