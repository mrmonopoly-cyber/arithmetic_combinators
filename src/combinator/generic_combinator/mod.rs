pub mod basic_combinator{
    use crate::{combinator::GenericCombinator, graph::CombGraph};

    type PortType<'a> = Option<&'a BasicCombinator<'a>>;
    type AuxPortType<'a> = Option<&'a [PortType<'a>]>;

    #[derive(Debug)]
    pub struct BasicCombinator<'a> {
        name: &'a str,
        label: u8,
        main_port: PortType<'a>,
        auxi_port: AuxPortType<'a>,
        rules: Option<&'a[CombRule<'a>]>,
    }

    #[derive(Debug)]
    pub struct CombRule<'a> {
        pub prim_port_label: u8,
        pub aux_port_labels: Option<&'a[Option<u8>]>,
        pub substitution: CombGraph<'a>,
    }

    impl <'a> BasicCombinator<'a> {
        pub fn new_basic_combinator(label: u8, 
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
                    (Some(s_aux_port),Some(r_aux_port)) => false, //to finis
                    _ => false
                }
            }else{
                false
            }
        }
    }

    impl <'a> GenericCombinator for BasicCombinator<'a>{
        fn get_lable_id(&self) -> u8 {
            self.label
        }

        fn get_lable_name(&self) -> &str {
            self.name
        }

        fn compute(&self) -> Option<CombGraph<'a>> {
            if let Some(node) = self.main_port{
                if let Some(rules) = self.rules{
                    for rul in rules {
                        if self.can_apply(node, rul){
                            return Some(rul.substitution)
                        }
                    }
                }
            }
            None
        }
    }
}
