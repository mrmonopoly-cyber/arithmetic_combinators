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
        CLONE,
        ZERO,
        POS,
        NEG,
        INC,
        DEC,
        SUM,
        MULT,
        DIV,
        DIVINV,
        SIGN,
        NATU,
        ERASER,
        LAST,
    }

    fn create_op<'a>(op: ArithOp) -> Operation<'a> {
        match op {
           ArithOp::ZERO => Operation::new(1, "ZERO"),
           ArithOp::POS => Operation::new(2, "POS"),
           ArithOp::NEG => Operation::new(2, "NEG"),
           ArithOp::INC => Operation::new(2, "INC"),
           ArithOp::DEC => Operation::new(2, "DEC"),
           ArithOp::SUM => Operation::new(3, "SUM"),
           ArithOp::MULT=> Operation::new(3, "MULT"),
           ArithOp::DIV => Operation::new(3, "DIV"),
           ArithOp::CLONE=> Operation::new(3, "CLONE"),
           ArithOp::SIGN=> Operation::new(2, "SIGN"),
           ArithOp::NATU=> Operation::new(2, "NATU"),
           ArithOp::ERASER=> Operation::new(1, "ERASER"),
           ArithOp::LAST=> Operation::new(2, "LAST"),
           ArithOp::DIVINV=> Operation::new(3, "DIV_INV"),
        }

    }

    fn get_arith_ops<'a>() -> Box<[Operation<'a>]>  {
        let mut res = Vec::with_capacity(ArithOp::VARIANT_COUNT);
        for op in ArithOp::iter(){
            res.push(create_op(op));
        };
        res.into_boxed_slice()
    }

    fn add_all_out_rule_arity_2<'a>(mut op_pool: OpPool<'a>,
        main_op : &'a str,
        aux_op: &'a str,
        sub: SubPattern<'a> ) -> OpPool<'a>{

        op_pool.add_rule( main_op, ([None,Some(aux_op)].as_slice(),sub.clone()));
        for op in get_arith_ops().iter(){
            op_pool.add_rule( main_op, ([Some(op.label),Some(aux_op)].as_slice(),sub.clone()));
        }

        op_pool
    }

    fn add_all_out_rule_arity_3<'a>(mut op_pool: OpPool<'a>,
        main_op : &'a str,
        aux_op: &'a str,
        aux_op_1: &'a str,
        sub: SubPattern<'a> ) -> OpPool<'a>{

        op_pool.add_rule( main_op, ([None,Some(aux_op), Some(aux_op_1)].as_slice(),sub.clone()));
        for op in get_arith_ops().iter(){
            op_pool.add_rule( main_op, ([Some(op.label),Some(aux_op), Some(aux_op_1)].as_slice(),sub.clone()));
        }

        op_pool
    }

    fn add_dec_rules(mut op_pool: OpPool) -> OpPool{
        let zero_dec_sub: SubPattern = SubPattern{
            new_nodes_labels: &["ZERO","NEG"],
            int_links: &[&SubIntLink{ start: 0, dst: 1, start_port: 0,end_port: 0,}],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1, port: 1},
            ]),
            result_node: 1,
        };

        let neg_dec_sub: SubPattern = SubPattern{
            new_nodes_labels: &["NEG","NEG"],
            int_links: &[&SubIntLink{ start: 0, dst: 1, start_port: 1,end_port: 0,}],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 0, port: 0},
                SubFreePort{node: 1, port: 1},
            ]),
            result_node: 1,
        };

        let pos_dec_sub: SubPattern = SubPattern{
            new_nodes_labels: &[],
            int_links: &[],
            ext_links: Some(&[
                (0,1),
            ]),
            free_ports: None,
            result_node: 1,
        };

        op_pool = add_all_out_rule_arity_2(op_pool, "DEC", "ZERO" , zero_dec_sub);
        op_pool = add_all_out_rule_arity_2(op_pool, "DEC", "NEG" , neg_dec_sub);
        op_pool = add_all_out_rule_arity_2(op_pool, "DEC", "POS" , pos_dec_sub);

        op_pool
    }

    fn add_inc_rules(mut op_pool: OpPool) -> OpPool{
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

        let neg_inc_sub: SubPattern = SubPattern{
            new_nodes_labels: &[],
            int_links: &[],
            ext_links: Some(&[
                (0,1),
            ]),
            free_ports: None,
            result_node: 1,
        };


        op_pool = add_all_out_rule_arity_2(op_pool, "INC", "ZERO" , zero_inc_sub);
        op_pool = add_all_out_rule_arity_2(op_pool, "INC", "POS" , pos_inc_sub);
        op_pool = add_all_out_rule_arity_2(op_pool, "INC", "NEG" , neg_inc_sub);

        op_pool
    }

    fn add_sum_rules(mut op_pool: OpPool) -> OpPool{
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

        let dec_zero_sum_sub: SubPattern = SubPattern{
            new_nodes_labels: &["SUM","DEC"],
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

        op_pool = add_all_out_rule_arity_3(op_pool, "SUM", "POS", "ZERO"  , inc_zero_sum_sub.clone());
        op_pool = add_all_out_rule_arity_3(op_pool, "SUM", "POS", "POS"  , inc_zero_sum_sub.clone());
        op_pool = add_all_out_rule_arity_3(op_pool, "SUM", "POS", "NEG"  , inc_zero_sum_sub);

        op_pool = add_all_out_rule_arity_3(op_pool, "SUM", "NEG", "ZERO"  , dec_zero_sum_sub.clone());
        op_pool = add_all_out_rule_arity_3(op_pool, "SUM", "NEG", "NEG"  , dec_zero_sum_sub.clone());
        op_pool = add_all_out_rule_arity_3(op_pool, "SUM", "NEG", "POS"  , dec_zero_sum_sub.clone());

        op_pool = add_all_out_rule_arity_3(op_pool, "SUM", "ZERO", "ZERO"  , zero_zero_sum_sub.clone());
        op_pool = add_all_out_rule_arity_3(op_pool, "SUM", "ZERO", "NEG"  , zero_zero_sum_sub.clone());
        op_pool = add_all_out_rule_arity_3(op_pool, "SUM", "ZERO", "POS"  , zero_zero_sum_sub);

        op_pool
    }

    fn add_mult_rules(mut op_pool: OpPool) -> OpPool{
        let pos_zero_rule = SubPattern {
            new_nodes_labels: &["ZERO","ZERO"],
            int_links : &[],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 0, port: 0},
                SubFreePort{node: 1, port: 0},
            ]),
            result_node : 1,
        };

        let zero_pos_rule = SubPattern {
            new_nodes_labels: &[],
            int_links : &[],
            ext_links: Some(&[
                (0,2),
            ]),
            free_ports: None,
            result_node : 0,
        };

        let pos_pos_rule = SubPattern {
            new_nodes_labels: &["MULT","CLONE","SUM"],
            int_links : &[
                &SubIntLink{start: 1, start_port: 0, dst: 0, end_port: 2},
                &SubIntLink{start: 1, start_port: 2, dst: 2, end_port: 2},
                &SubIntLink{start: 0, start_port: 0, dst: 2, end_port: 1},
            ],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1,port: 1},
                SubFreePort{node: 0,port: 1},
                SubFreePort{node: 2,port: 0},
            ]),
            result_node : 2,
        };

        let neg_neg_rule = SubPattern {
            new_nodes_labels: &["MULT","SIGN","POS","SIGN"],
            int_links : &[
                &SubIntLink{start: 0, start_port: 2, dst: 1, end_port: 0},
                &SubIntLink{start: 0, start_port: 1, dst: 2, end_port: 1},
                &SubIntLink{start: 3, start_port: 0, dst: 2, end_port: 0},
            ],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1,port: 1},
                SubFreePort{node: 3,port: 1},
                SubFreePort{node: 0,port: 0},
            ]),
            result_node : 0,
        };

        let pos_neg_rule = SubPattern {
            new_nodes_labels: &["MULT","SIGN","POS","SIGN"],
            int_links : &[
                &SubIntLink{start: 0, start_port: 2, dst: 1, end_port: 0},
                &SubIntLink{start: 0, start_port: 1, dst: 2, end_port: 1},
                &SubIntLink{start: 3, start_port: 1, dst: 0, end_port: 0},
            ],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1,port: 1},
                SubFreePort{node: 2,port: 0},
                SubFreePort{node: 3,port: 0},
            ]),
            result_node : 3,
        };

        let neg_pos_rule = SubPattern {
            new_nodes_labels: &["MULT","SIGN","NEG","SIGN"],
            int_links : &[
                &SubIntLink{start: 0, start_port: 2, dst: 1, end_port: 0},
                &SubIntLink{start: 0, start_port: 1, dst: 2, end_port: 1},
                &SubIntLink{start: 3, start_port: 1, dst: 0, end_port: 0},
            ],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1,port: 1},
                SubFreePort{node: 2,port: 0},
                SubFreePort{node: 3,port: 0},
            ]),
            result_node : 3,
        };

        op_pool = add_all_out_rule_arity_3(op_pool, "MULT", "ZERO", "POS", pos_zero_rule.clone());
        op_pool = add_all_out_rule_arity_3(op_pool, "MULT", "ZERO", "NEG", pos_zero_rule);

        op_pool = add_all_out_rule_arity_3(op_pool, "MULT", "POS", "ZERO", zero_pos_rule.clone());
        op_pool = add_all_out_rule_arity_3(op_pool, "MULT", "NEG", "ZERO", zero_pos_rule);

        op_pool = add_all_out_rule_arity_3(op_pool, "MULT", "POS", "POS", pos_pos_rule);
        op_pool = add_all_out_rule_arity_3(op_pool, "MULT", "NEG", "NEG", neg_neg_rule);

        op_pool = add_all_out_rule_arity_3(op_pool, "MULT", "POS", "NEG", pos_neg_rule);
        op_pool = add_all_out_rule_arity_3(op_pool, "MULT", "NEG", "POS", neg_pos_rule);

        op_pool
    }


    fn add_copy_rules<'a>(mut op_pool: OpPool<'a>) -> OpPool{
        let copy_pos_sub = SubPattern{
            new_nodes_labels: &["CLONE","POS","POS"],
            int_links : &[
                &SubIntLink{start: 0, start_port:2, dst: 1, end_port: 0},
                &SubIntLink{start: 0, start_port:0, dst: 2, end_port: 0},
            ],
            ext_links : None,
            free_ports: Some(&[
                SubFreePort{node: 1, port: 1},
                SubFreePort{node: 0, port: 1},
                SubFreePort{node: 2, port: 1},
            ]),
            result_node: 0,
        };

        let copy_neg_sub = SubPattern{
            new_nodes_labels: &["CLONE","NEG","NEG"],
            int_links : &[
                &SubIntLink{start: 0, start_port:2, dst: 1, end_port: 0},
                &SubIntLink{start: 0, start_port:0, dst: 2, end_port: 0},
            ],
            ext_links : None,
            free_ports: Some(&[
                SubFreePort{node: 1, port: 1},
                SubFreePort{node: 0, port: 1},
                SubFreePort{node: 2, port: 1},
            ]),
            result_node: 0,
        };

        let copy_zero_sub = SubPattern{
            new_nodes_labels: &["ZERO","ZERO"],
            int_links : &[],
            ext_links : None,
            free_ports: Some(&[
                SubFreePort{node: 0, port: 0},
                SubFreePort{node: 1, port: 0},
            ]),
            result_node: 0,
        };

        op_pool.add_rule( "CLONE", ([Some("POS"),Some("POS"), Some("POS")].as_slice(),copy_pos_sub.clone()));
        op_pool.add_rule( "CLONE", ([Some("NEG"),Some("NEG"), Some("NEG")].as_slice(),copy_neg_sub.clone()));

        op_pool.add_rule( "CLONE", ([Some("NEG"),Some("ZERO"), Some("NEG")].as_slice(),copy_zero_sub.clone()));
        op_pool.add_rule( "CLONE", ([Some("POS"),Some("ZERO"), Some("POS")].as_slice(),copy_zero_sub.clone()));

        op_pool.add_rule( "CLONE", ([Some("SIGN"),Some("POS"), Some("DIV")].as_slice(),copy_pos_sub.clone()));
        
        op_pool.add_rule( "CLONE", ([Some("NATU"),Some("POS"), Some("LAST")].as_slice(),copy_pos_sub.clone()));
        op_pool.add_rule( "CLONE", ([Some("NATU"),Some("NEG"), Some("LAST")].as_slice(),copy_neg_sub.clone()));
        op_pool.add_rule( "CLONE", ([Some("NATU"),Some("ZERO"), Some("LAST")].as_slice(),copy_zero_sub.clone()));

        op_pool
    }

    fn add_sign_rules(mut op_pool: OpPool) -> OpPool{
        let pos_rule = SubPattern{
            new_nodes_labels: &["NEG","SIGN"],
            int_links : &[&SubIntLink{start: 0,start_port: 0, dst: 1, end_port: 0}],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1, port: 1},
                SubFreePort{node: 0, port: 1},
            ]),
            result_node: 0,
        };

        let neg_rule = SubPattern{
            new_nodes_labels: &["POS","SIGN"],
            int_links : &[&SubIntLink{start: 0,start_port: 0, dst: 1, end_port: 0}],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1, port: 1},
                SubFreePort{node: 0, port: 1},
            ]),
            result_node: 0,
        };

        let zero_rule = SubPattern{
            new_nodes_labels: &["ZERO"],
            int_links : &[],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 0,port: 0},
            ]),
            result_node: 0,
        };

        op_pool = add_all_out_rule_arity_2(op_pool, "SIGN" , "POS", pos_rule);
        op_pool = add_all_out_rule_arity_2(op_pool, "SIGN" , "NEG", neg_rule);
        op_pool = add_all_out_rule_arity_2(op_pool, "SIGN" , "ZERO", zero_rule);

        op_pool
    }

    fn add_natu_rules(mut op_pool: OpPool) -> OpPool{

        let pass_zero_rule = SubPattern{
            new_nodes_labels: &["ZERO"],
            int_links: &[],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 0, port: 0},
            ]),
            result_node: 0,
        };

        let pass_pos_rule = SubPattern{
            new_nodes_labels: &["POS"],
            int_links: &[],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 0, port: 0},
                SubFreePort{node: 0, port: 1},
            ]),
            result_node: 0,
        };

        let block_rule = SubPattern{
            new_nodes_labels: &["ZERO","ERASER"],
            int_links: &[],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1, port: 0},
                SubFreePort{node: 0, port: 0},
            ]),
            result_node: 0,
        };

        op_pool = add_all_out_rule_arity_2(op_pool, "NATU" , "ZERO", pass_zero_rule);
        op_pool = add_all_out_rule_arity_2(op_pool, "NATU" , "POS", pass_pos_rule);
        op_pool = add_all_out_rule_arity_2(op_pool, "NATU" , "NEG", block_rule);

        op_pool
    }

    fn add_div_rules(mut op_pool: OpPool) -> OpPool{

        let zero_div = SubPattern{
            new_nodes_labels: &["ZERO","ZERO","ZERO"],
            int_links: &[],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 0, port: 0},
                SubFreePort{node: 1, port: 0},
                SubFreePort{node: 2, port: 0},
            ]),
            result_node: 2,
        };

        let pos_pos = SubPattern{
            new_nodes_labels: &["POS","SUM","SIGN","NATU","CLONE","CLONE","DIV","LAST","SUM"],
            int_links: &[
                //SUM
                &SubIntLink{start: 1, dst: 0,start_port: 1, end_port: 1},
                &SubIntLink{start: 1, dst: 2,start_port: 2, end_port: 0},
                &SubIntLink{start: 1, dst: 5,start_port: 0, end_port: 1},
                //CLONE
                &SubIntLink{start: 4, start_port: 0, dst: 2, end_port: 1},
                &SubIntLink{start: 4, start_port: 2, dst: 6, end_port: 2},
                //CLONE
                &SubIntLink{start: 5, start_port: 2, dst: 7, end_port: 1},
                &SubIntLink{start: 5, start_port: 0, dst: 3, end_port: 1},
                //DIV
                &SubIntLink{start: 6, start_port: 1, dst: 3, end_port: 0},
                //SUM
                &SubIntLink{start: 8, start_port: 2, dst: 7, end_port: 0},
                &SubIntLink{start: 8, start_port: 1, dst: 6, end_port: 0},
            ],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 4, port: 1},
                SubFreePort{node: 0, port: 0},
                SubFreePort{node: 8, port: 0},
            ]),
            result_node: 8,
        };

        op_pool = add_all_out_rule_arity_3(op_pool, "DIV", "ZERO", "POS", zero_div.clone() );
        op_pool = add_all_out_rule_arity_3(op_pool, "DIV", "ZERO", "NEG", zero_div );

        op_pool = add_all_out_rule_arity_3(op_pool, "DIV", "POS", "POS", pos_pos);

        op_pool
    }

    fn add_last_rules(mut op_pool: OpPool) -> OpPool{
        let last_pos = SubPattern{
            new_nodes_labels: &["ERASER","POS","ZERO"],
            int_links : &[&SubIntLink{start: 1, dst: 2, start_port: 0, end_port: 0}],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 0,port: 0},
                SubFreePort{node: 1,port: 1},
            ]),
            result_node: 1,
        };

        let last_neg = SubPattern{
            new_nodes_labels: &["ERASER","ZERO"],
            int_links : &[],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 0,port: 0},
                SubFreePort{node: 1,port: 0},
            ]),
            result_node: 1,
        };

        let last_zero = SubPattern{
            new_nodes_labels: &["ERASER","POS","ZERO"],
            int_links : &[&SubIntLink{start: 1, dst: 2, start_port: 0, end_port: 0}],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1,port: 1},
                SubFreePort{node: 0,port: 0},
            ]),
            result_node: 1,
        };

        op_pool = add_all_out_rule_arity_2(op_pool, "LAST", "POS", last_pos);
        op_pool = add_all_out_rule_arity_2(op_pool, "LAST", "NEG", last_neg);
        op_pool = add_all_out_rule_arity_2(op_pool, "LAST", "ZERO", last_zero);

        op_pool
    }

    fn add_div_inv_rules(mut op_pool: OpPool) -> OpPool{

        let div_zero = SubPattern{
            new_nodes_labels: &["ZERO","DIV"],
            int_links : &[&SubIntLink{start: 0, dst: 1, start_port: 0, end_port: 2}],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1,port: 1},
                SubFreePort{node: 1,port: 0},
            ]),
            result_node: 1,
        };

        let div_pos = SubPattern{
            new_nodes_labels: &["POS","DIV"],
            int_links : &[&SubIntLink{start: 0, dst: 1, start_port: 1, end_port: 2}],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1,port: 1},
                SubFreePort{node: 0,port: 0},
                SubFreePort{node: 1,port: 0},
            ]),
            result_node: 1,
        };

        let div_neg = SubPattern{
            new_nodes_labels: &["NEG","DIV"],
            int_links : &[&SubIntLink{start: 0, dst: 1, start_port: 1, end_port: 2}],
            ext_links: None,
            free_ports: Some(&[
                SubFreePort{node: 1,port: 1},
                SubFreePort{node: 0,port: 0},
                SubFreePort{node: 1,port: 0},
            ]),
            result_node: 1,
        };

        op_pool = add_all_out_rule_arity_3(op_pool, "DIV_INV", "ZERO", "POS", div_zero.clone());
        op_pool = add_all_out_rule_arity_3(op_pool, "DIV_INV", "ZERO", "NEG", div_zero);

        op_pool = add_all_out_rule_arity_3(op_pool, "DIV_INV", "POS", "POS", div_pos.clone());
        op_pool = add_all_out_rule_arity_3(op_pool, "DIV_INV", "POS", "NEG", div_pos.clone());

        op_pool = add_all_out_rule_arity_3(op_pool, "DIV_INV", "NEG", "NEG", div_neg.clone());
        op_pool = add_all_out_rule_arity_3(op_pool, "DIV_INV", "NEG", "POS", div_neg.clone());

        op_pool
    }
    
    fn new_graph() -> Graph<'static > {
        let mut op_pool = OpPool::new(get_arith_ops());

        op_pool = add_copy_rules(op_pool);
        op_pool = add_sign_rules(op_pool);
        op_pool = add_natu_rules(op_pool);
        op_pool = add_last_rules(op_pool);
        op_pool = add_div_rules(op_pool);

        op_pool = add_inc_rules(op_pool);
        op_pool = add_dec_rules(op_pool);
        op_pool = add_sum_rules(op_pool);
        op_pool = add_mult_rules(op_pool);
        op_pool = add_div_inv_rules(op_pool);

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
                '*' => GRAPH.attach("MULT"),
                '/' => GRAPH.attach("DIV_INV"),
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

    pub fn get_result() -> Option<i32>{
        unsafe {
            GRAPH.get_result()
        }
    }

    pub fn print_rules(){
        unsafe {
            GRAPH.print_rules();
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
