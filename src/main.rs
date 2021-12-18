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
use codegen::{Codegen, Value, BcArr};
use vm::Interpreter;

const DEBUG: bool = true;

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

    if DEBUG {
        println!("\n+-----------Source-Code------------+");
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
            println!("{}", "\nCould not compile program due to errors\n"
                     .red().bold()); 
            return;
        }
    };
    //print!("Statements: {:#?}", stmts);

    let (bytecode, const_pool) = Codegen::bytecode_gen(stmts);

    if DEBUG {
        print!("+-----------Bytecode--------------+");
        for instr in bytecode.clone() {
            match instr.clone() {
                BcArr::I(v) => { print!("\n{:?}   ", v) },
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
            println!("\n+----------------------------------+\n");
        } else { println!("\n+----------------------------------+\n"); }
    }
    let mut vm = Interpreter::new(bytecode, const_pool);
    vm.interpret();
}
