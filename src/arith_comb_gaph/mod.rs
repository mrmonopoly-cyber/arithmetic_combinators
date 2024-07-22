mod graph;
mod operation;
mod operation_pool;


pub mod arith_combinator_graph{
    static mut GRAPH: once_cell::sync::Lazy<Graph> = once_cell::sync::Lazy::new(|| new_graph());

    use super::
    {
        graph::graph::Graph, operation::operations::Operation, operation_pool::operation_pool::{OpPool, SubFreePort, SubIntLink, SubPattern}
    };
    use strum::IntoEnumIterator;
    use variant_count::VariantCount;
    use strum_macros::EnumIter;

    #[derive(EnumIter,VariantCount)]
    enum ArithOp {
        ZERO,
        POS,
        NEG,
        INC,
        DEC,
        SUM,
    }

    fn create_op<'a>(op: ArithOp) -> Operation<'a> {
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

    fn add_inc_rules(op_pool: &mut OpPool){
        let zero_inc_sub: SubPattern = SubPattern{
            new_nodes_labels: &["ZERO","POS"],
            int_links: &[&SubIntLink{ start: 0, dst: 1, start_port: 0,end_port: 0,}],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1, port: 1},
            ]),
            result_node: 1,
        };

        let pos_inc_sub: SubPattern = SubPattern{
            new_nodes_labels: &["POS","POS"],
            int_links: &[&SubIntLink{ start: 0, dst: 1, start_port: 1,end_port: 0,}],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 0, port: 0},
                SubFreePort{node: 1, port: 1},
            ]),
            result_node: 1,
        };

        op_pool.add_rule( "INC", ([None,Some("ZERO")].as_slice(),zero_inc_sub.clone()));
        op_pool.add_rule( "INC", ([Some("INC"),Some("ZERO")].as_slice(),zero_inc_sub.clone()));
        op_pool.add_rule( "INC", ([Some("SUM"),Some("ZERO")].as_slice(),zero_inc_sub));

        op_pool.add_rule( "INC", ([None,Some("POS")].as_slice(),pos_inc_sub.clone()));
        op_pool.add_rule( "INC", ([Some("INC"),Some("POS")].as_slice(),pos_inc_sub.clone()));
        op_pool.add_rule( "INC", ([Some("SUM"),Some("POS")].as_slice(),pos_inc_sub));
    }

    fn add_sum_rules(op_pool: &mut OpPool){
        let inc_zero_sum_sub: SubPattern = SubPattern{
            new_nodes_labels: &["SUM","INC"],
            int_links: &[&SubIntLink{ start: 0, dst: 1, start_port: 0,end_port: 1,}],
            ext_links: None,
            free_ports:Some( &[
                SubFreePort{node: 0, port: 2},
                SubFreePort{node: 0, port: 1},
                SubFreePort{node: 1, port: 0},
            ]),
            result_node: 1,
        };

        let zero_zero_sum_sub: SubPattern = SubPattern{
            new_nodes_labels: &[],
            int_links: &[],
            ext_links: Some(&[
                (0,1),
            ]),
            free_ports: None,
            result_node: 0,
        };

        op_pool.add_rule( "SUM", ([None,Some("POS"),Some("ZERO")].as_slice(),inc_zero_sum_sub.clone()));
        op_pool.add_rule( "SUM", ([None,Some("POS"),Some("POS")].as_slice(),inc_zero_sum_sub.clone()));
        op_pool.add_rule( "SUM", ([Some("INC"),Some("POS"),Some("POS")].as_slice(),inc_zero_sum_sub.clone()));
        op_pool.add_rule( "SUM", ([Some("SUM"),Some("POS"),Some("POS")].as_slice(),inc_zero_sum_sub));

        op_pool.add_rule( "SUM", ([None,Some("ZERO"),Some("ZERO")].as_slice(),zero_zero_sum_sub.clone()));
        op_pool.add_rule( "SUM", ([Some("INC"),Some("ZERO"),Some("POS")].as_slice(),zero_zero_sum_sub));
    }
    
    fn new_graph() -> Graph<'static > {
        let mut op_pool = OpPool::new(get_arith_ops());



        add_inc_rules(&mut op_pool);
        add_sum_rules(&mut op_pool);

        Graph::new(op_pool)
    }

    pub fn push_num(mut num : i32){
        unsafe {
            if num > 0{
                while num != 0{
                    GRAPH.attach("INC");
                    num-=1;
                };
            }else if num < 0{
                while num != 0{
                    GRAPH.attach("DEC");
                    num+=1;
                };
            }
            GRAPH.attach("ZERO");
        }
    }

    pub fn push_op(op : char){
        unsafe {
            match op {
                '+' => GRAPH.attach("SUM"),
                _ => {
                    println!("operation not implemented");
                    false
                }
            };
        }
    }

    pub fn compute(){
        unsafe {
            GRAPH.compute();
        }
    }

    pub fn print_graph(){
        unsafe {
            GRAPH.print_graph();
        }
    }

    pub fn reset(){
        unsafe {
            GRAPH.clear();
        }
    }
}
