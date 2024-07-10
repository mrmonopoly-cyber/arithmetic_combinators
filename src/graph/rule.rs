pub mod rule{
    use crate::graph::generic_combinator::basic_combinator::BasicCombinator;

    #[derive(Debug)]
    pub struct RuleComb<'a>{
        label_active_node: u8,
        label_second_node: u8,
        conf_nodes: [&'a [u8];2],
        subst: &'a[BasicCombinator<'a>]
    }
}
