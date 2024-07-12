mod graph;
mod operation;
mod operation_pool;

pub mod arith_combinator_graph{
    use super::
    {
        graph::graph::Graph, 
        operation::operations::Operation, 
        operation_pool::operation_pool::OpPool,
    };
    use strum::IntoEnumIterator;
    use variant_count::VariantCount;
    use strum_macros::EnumIter;

    #[derive(EnumIter,VariantCount)]
    pub enum ArithOp {
        ZERO,
        INC,
        DEC,
        SUM,
    }

    pub fn create_op<'a>(op: ArithOp) -> Operation<'a> {
        match op {
           ArithOp::ZERO => Operation::new(1, "ZERO"),
           ArithOp::INC => Operation::new(2, "INC"),
           ArithOp::DEC => Operation::new(2, "DEC"),
           ArithOp::SUM => Operation::new(3, "SUM"),
        }

    }

    fn get_arith_ops<'a>() -> Box<[Operation<'a>]>  {
        let mut res = Vec::with_capacity(ArithOp::VARIANT_COUNT);
        for op in ArithOp::iter(){
            res.push(create_op(op));
        };

        res.into_boxed_slice()
    }

    pub fn new_graph<'a>() -> Graph<'a> {
        let mut op_pool = OpPool::new(get_arith_ops());
        op_pool.add_rule("INC", "ZERO", None);
        op_pool.add_rule("INC", "INC", None);

        let sum_cond = Box::new(
            [
                [Some("INC"),Some("INC"),None].as_slice(),
                [Some("INC"),Some("INC"),None].as_slice(),
                [Some("INC"),Some("DEC"),None].as_slice(),
                [Some("INC"),Some("ZERO"),None].as_slice(),
                [Some("DEC"),Some("INC"),None].as_slice(),
                [Some("DEC"),Some("DEC"),None].as_slice(),
                [Some("DEC"),Some("ZERO"),None].as_slice(),
                [Some("ZERO"),Some("INC"),None].as_slice(),
                [Some("ZERO"),Some("DEC"),None].as_slice(),
                [Some("ZERO"),Some("ZERO"),None].as_slice(),
            ]);

        op_pool.add_rule("SUM", "INC", Some(sum_cond));
        Graph::new(op_pool)
    }
}
