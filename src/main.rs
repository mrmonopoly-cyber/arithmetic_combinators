mod arith_comb_gaph;

use std::io;
use std::io::Write;

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
    let lexer_rules = lexer_rules();
    let grammar = grammar();

    let mut input = String::new();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    loop{
        write!(handle, "CLI>: ").expect("Failed to write prompt");
        handle.flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().is_empty() {
            continue;
        }

        let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
        let parse_tree = &santiago::parser::parse(&grammar, &lexemes).unwrap()[0];
        let ast = parse_tree.as_abstract_syntax_tree();

        eval(&ast);

        print_graph();

        compute();
        match get_result(){
            None => writeln!(handle, "computation failed").expect("Failed to write error"),
            Some(r) => {
                let mut res = "res".to_owned();
                res.push_str(&r.to_string());
                writeln!(handle, "res := {}", r).expect("Failed to write result");
            },
        };
        handle.flush().ok();
        handle.flush().expect("Failed to flush stdout");

        reset();
    }
}
