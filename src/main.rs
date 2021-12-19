mod lexer;
mod tokens;
mod parser;
mod ast;
mod err;
mod codegen;
mod vm;

extern crate colored;

use colored::*;
use std::{fs};
use std::env;
use lexer::{tokenize};
use parser::{Parser};
use codegen::{Codegen, Value, BcArr, Instr};
use vm::Interpreter;

const DEBUGSOURCE: bool = true;
const DEBUGAST: bool = true;
const DEBUGBYTECODE: bool = true;

fn print_line(file: String, line: u32) {
    let mut count: u32 = 0;
    print!("\n");
    for c in file.chars() {
        if count == line-1 {
            print!("{}", c);
        } 
        if c == '\n' { count +=1; }
    }
}

/// Read content from a file into a string and pass that string
/// to Token::tokenize
fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide your .js file as the sole argument");
        return ();
    }
    
    // Get file_name from argv and read the entire file into file_string
    let file_name = args[1].to_string();
    let file_string = fs::read_to_string(file_name)
        .expect("Unable to read file");

    if DEBUGSOURCE {
        println!("\n+-----------Source-Code-----------+");
        println!("{}", file_string);
    }

    #[allow(unused_mut)]
    let mut tokens = tokenize(&file_string);
    //println!("Successfuly generated Tokens: \n {:#?}\nEND", tokens);
    
    let mut parser = Parser::new(tokens);
    let stmts  = match parser.parse() {
        Ok(stmts) => stmts,
        Err(err)  => { 
            for e in err {
                print_line(file_string.clone(), e.line);
                println!("{}\n\n", e.err.bold());
            }
            println!("{}", "Could not compile program due to errors\n"
                     .red().bold()); 
            return;
        }
    };

    if DEBUGAST {
        println!("+----------------AST-----------------+");
        for stmt in stmts.clone() {
            println!("{:#?}", stmt);
        }
    }

    let (bytecode, const_pool) = Codegen::bytecode_gen(stmts);

    if DEBUGBYTECODE {
        print!("+-----------Bytecode--------------+");
        for (j, instr) in bytecode.clone().iter().enumerate() {
            let i = j+1;
            match instr.clone() {
                BcArr::I(Instr::Add) => { print!("\n{:4}   Add     ", i) },
                BcArr::I(Instr::Sub) => { print!("\n{:4}   Sub     ", i) },
                BcArr::I(Instr::Div) => { print!("\n{:4}   Div     ", i) },
                BcArr::I(Instr::Mul) => { print!("\n{:4}   Mul     ", i) },
                BcArr::I(v) => { print!("\n{:4}   {:?}   ", i, v) },
                BcArr::V(Value::Number(v)) => { print!("{:?}, ", v) },
                BcArr::V(Value::Reg(v)) => { print!("{:?}, ", Value::Reg(v)) },
                BcArr::V(Value::Pool(v)) => { print!("{:?}, ", Value::Pool(v)) },
                BcArr::V(Value::StringLiteral(v)) => { print!("{:?}, ", v) },
                BcArr::V(Value::CPool(v)) => { print!("{:?}, ", Value::CPool(v)) },
                BcArr::V(Value::Bool(v)) => { print!("{:?}, ", Value::Bool(v)) },
                BcArr::V(Value::VAddr(v)) => { print!("{:?}, ", Value::VAddr(v)) },
                _ => { panic!("print stuff"); },
            } }
        if const_pool.len() > 0 {
            println!("\n+-----------Const-Pool-------------+\n");
            for (i,c) in const_pool.clone().iter().enumerate() {
                println!("[{}] - {:?}", i, c);
            }
        }
        println!("\n+----------------------------------+\n");
    }
    let mut vm = Interpreter::new(bytecode, const_pool);
    vm.interpret();
}
