mod arith_comb_gaph;

use santiago::lexer::LexerRules;
use santiago::grammar::Associativity;
use santiago::grammar::Grammar;
use arith_comb_gaph::arith_combinator_graph::*;


#[derive(Debug)]
pub enum AST {
    Int(i32),
    BinaryOperation(Vec<AST>),
    UnaryOperator(Vec<AST>),
    OperatorAdd,
    OperatorSubtract,
    OperatorMultiply,
    OperatorDivide,
    OperatorOpenPar,
    OperatorClosePar,
}

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "INT" = pattern r"[0-9]+";
        "DEFAULT" | "+" = string "+";
        "DEFAULT" | "-" = string "-";
        "DEFAULT" | "*" = string "*";
        "DEFAULT" | "/" = string "/";
        "DEFAULT" | "(" = string "(";
        "DEFAULT" | ")" = string ")";
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    )
}

pub fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "expr" => rules "int";

        "expr" => rules "expr" "add" "expr" =>
            AST::BinaryOperation;
        "expr" => rules "expr" "subtract" "expr" =>
            AST::BinaryOperation;
        "expr" => rules "expr" "multiply" "expr" =>
            AST::BinaryOperation;
        "expr" => rules "expr" "divide" "expr" =>
            AST::BinaryOperation;
        "expr" => rules "open_par" "expr" "close_par" =>
            AST::BinaryOperation;

        "expr" => rules "subtract" "expr" =>
            AST::UnaryOperator;

        "add" => lexemes "+" =>
            |_| AST::OperatorAdd;
        "subtract" => lexemes "-" =>
            |_| AST::OperatorSubtract;
        "multiply" => lexemes "*" =>
            |_| AST::OperatorMultiply;
        "divide" => lexemes "/" =>
            |_| AST::OperatorDivide;

        "open_par" => lexemes "(" =>
            |_| AST::OperatorOpenPar;

        "close_par" => lexemes ")" =>
            |_| AST::OperatorClosePar;


        "int" => lexemes "INT" =>
            |lexemes| {
                let value = str::parse(&lexemes[0].raw).unwrap();
                AST::Int(value)
            };

        Associativity::Left => rules "add" "subtract";
        Associativity::Left => rules "multiply" "divide";
    )
}

pub fn eval(value: &AST){
    match value {
        AST::Int(int) => push_num(*int),
        AST::BinaryOperation(args) => 
        {
            match &args[1] {
                AST::OperatorAdd => {
                    push_op('+');
                    eval(&args[0]);
                    eval(&args[2]);
                },
                AST::OperatorSubtract => {
                    push_op('-');
                    eval(&args[0]);
                    eval(&args[2]);
                },
                AST::OperatorMultiply =>{
                    push_op('*');
                    eval(&args[0]);
                    eval(&args[2]);
                },
                AST::OperatorDivide => {
                    push_op('/');
                    eval(&args[0]);
                    eval(&args[2]);
                },

                _ => eval(&args[1]),
            };
        },
        AST::UnaryOperator(args) =>
        {
            match &args[0]{
                AST::OperatorSubtract => push_op('-'),
                _ => unreachable!(),
            }
            push_num(0);
            eval(&args[1]);
        },
        _ => unreachable!(),
    }
}


fn main() {

    let input = "3 / (-1 + 8)";

    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();

    let grammar = grammar();
    let parse_tree = &santiago::parser::parse(&grammar, &lexemes).unwrap()[0];

    let ast = parse_tree.as_abstract_syntax_tree();

    eval(&ast);
    compute();
    match get_result(){
        None => println!("computation failed"),
        Some(r) => println!("res = {}",r),
    }
    reset();
}
