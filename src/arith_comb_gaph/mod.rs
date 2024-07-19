mod graph;
mod operation;
mod operation_pool;

pub mod arith_combinator_graph{
    use super::
    {
        graph::graph::Graph, operation::operations::Operation, operation_pool::operation_pool::{OpPool, SubFreePort, SubIntLink, SubPattern}
    };
    use strum::IntoEnumIterator;
    use variant_count::VariantCount;
    use strum_macros::EnumIter;

    #[derive(EnumIter,VariantCount)]
    pub enum ArithOp {
        ZERO,
        POS,
        NEG,
        INC,
        DEC,
        SUM,
    }

    pub fn create_op<'a>(op: ArithOp) -> Operation<'a> {
        match op {
           ArithOp::ZERO => Operation::new(1, "ZERO"),
           ArithOp::POS => Operation::new(2, "POS"),
           ArithOp::NEG => Operation::new(2, "NEG"),
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

    pub fn new_graph() -> Graph<'static > {
        let mut op_pool = OpPool::new(get_arith_ops());

        let zero_inc_sub: SubPattern = SubPattern{
            new_nodes_labels: &["ZERO","POS"],
            int_links: &[&SubIntLink{ start: 0, dst: 1, start_port: 0,end_port: 0,}],
            ext_links: &[],
            free_ports: &[
                SubFreePort{node: 1, port: 1},
            ],
            result_node: 1,
        };

        let pos_inc_sub: SubPattern = SubPattern{
            new_nodes_labels: &["POS","POS"],
            int_links: &[&SubIntLink{ start: 0, dst: 1, start_port: 1,end_port: 0,}],
            ext_links: &[],
            free_ports: &[
                SubFreePort{node: 0, port: 0},
                SubFreePort{node: 1, port: 1},
            ],
            result_node: 1,
        };

        op_pool.add_rule( "INC", ([None,Some("ZERO")].as_slice(),zero_inc_sub.clone()));
        op_pool.add_rule( "INC", ([Some("INC"),Some("ZERO")].as_slice(),zero_inc_sub));
        op_pool.add_rule( "INC", ([None,Some("POS")].as_slice(),pos_inc_sub));
        Graph::new(op_pool)
    }

}
