pub mod operation_pool{
    use super::super::operation::operations::*;

    #[derive(Debug)]
    pub struct OpPool<'a> {
        ops: Box<[Operation<'a>]>,
        rules: Option<Box<[Rule<'a>]>>,
    }

    #[derive(Debug,PartialEq)]
    pub struct Rule<'a> {
        other_active_rule: Operation<'a>,
        port_conf: Option<Box<[Operation<'a>]>>,
    }

    impl <'a> Rule<'a> {
    }

    impl<'a> OpPool<'a> {
        pub fn new(ops: Box<[Operation<'a>]>, rules: Option<Box<[Rule<'a>]>>) -> Self {
            Self{
                ops: ops,
                rules: rules,
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
    }
}
