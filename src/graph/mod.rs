mod generic_combinator;

pub mod zero;
pub mod inc;
pub mod dec;
pub mod rule;

pub mod graph{
    use super::generic_combinator::basic_combinator::BasicCombinator;

    #[derive(Debug)]
    pub struct GraphNet<'a> {
        cursor: Option<&'a BasicCombinator<'a>>,
        traceback: Vec<BasicCombinator<'a>>,
    }

    impl <'a> GraphNet<'a> {
        pub fn new() -> GraphNet<'a>{
            GraphNet{
                cursor: None,
                traceback: Vec::new(),
            }
        }

        pub fn add_node(mut self, node:&'a BasicCombinator<'a>) {
        }
        
    }

}
