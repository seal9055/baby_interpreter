mod ast;
mod codegen;
mod err;
mod lexer;
mod parser;
mod tokens;
mod vm;
mod ai;
mod comp_ai;

extern crate colored;

use codegen::{BcArr, Codegen, Instr, Value};
use colored::*;
use lexer::tokenize;
use parser::Parser;
use std::{env, fs};
use vm::Interpreter;
//use ai::AbstractInterpreter;
use comp_ai::AbstractInterpreter;

const DEBUGSOURCE: bool = true;
const DEBUGTOKENS: bool = false;
const DEBUGAST: bool = true;
const DEBUGBYTECODE: bool = true;

/// Used to print a line until \n (debug purposes)
fn print_line(file: String, line: u32) {
    let mut count: u32 = 0;
    println!();
    for c in file.chars() {
        if count == line - 1 {
            print!("{}", c);
        }
        if c == '\n' {
            count += 1;
        }
    }
}

/// Read source code (required syntax is similar to javascript) before passing
/// the code into the compilation pipeline:
/// 1. Lexer:       Split the source code into a series of tokens
/// 2. Parser:      Take the tokens and use them to create an AST
/// 3. Codegen:     Walk the AST and generate bytecode
/// 4. Interpreter: Iterate through the bytecode and execute the instructions
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide your .js file as the sole argument");
        return;
    }

    // Get file_name from argv and read the entire file into file_string
    let file_name = args[1].to_string();
    let file_string = fs::read_to_string(file_name).expect("Unable to read file");

    if DEBUGSOURCE {
        println!("\n+-----------Source-Code-----------+");
        println!("{}", file_string);
    }

    #[allow(unused_mut)]
    let mut tokens = tokenize(&file_string);

    if DEBUGTOKENS {
        println!("\n+-------------Tokens--------------+");
        for token in tokens.clone() {
            println!("{:?}", token);
        }
    }

    let mut parser = Parser::new(tokens);
    let stmts = match parser.parse() {
        Ok(stmts) => stmts,
        Err(err) => {
            for e in err {
                print_line(file_string.clone(), e.line);
                println!("{}\n\n", e.err.bold());
            }
            println!(
                "{}",
                "Could not compile program due to above errors\n"
                    .red()
                    .bold()
            );
            return;
        }
    };

    if DEBUGAST {
        println!("+----------------AST-----------------+");
        for stmt in stmts.clone() {
            println!("{:#?}", stmt);
        }
    }

    let program = Codegen::bytecode_gen(stmts);
    let mut vals = Vec::new();
    for (_, value) in program.clone().function_list.into_iter() {
        vals.push(value);
    }

    if DEBUGBYTECODE {
        print!("+-----------Bytecode--------------+");
        for (j, instr) in program.bytecode.iter().enumerate() {
            if vals.contains(&j) {
                for (key, value) in program.clone().function_list.into_iter() {
                    if value == j {
                        print!("\n\n\t< {} >", key);
                    }
                }
            }
            if j == program.entry_point {
                print!("\n\n\t< Entry Point >");
            }
            let i = j + 1;
            match instr.clone() {
                BcArr::I(Instr::Add) => {
                    print!("\n{:4}   Add     ", i)
                }
                BcArr::I(Instr::Sub) => {
                    print!("\n{:4}   Sub     ", i)
                }
                BcArr::I(Instr::Div) => {
                    print!("\n{:4}   Div     ", i)
                }
                BcArr::I(Instr::Mul) => {
                    print!("\n{:4}   Mul     ", i)
                }
                BcArr::I(Instr::Jmp) => {
                    print!("\n{:4}   Jmp     ", i)
                }
                BcArr::I(Instr::Call) => {
                    print!("\n{:4}   Call    ", i)
                }
                BcArr::I(v) => {
                    print!("\n{:4}   {:?}   ", i, v)
                }
                BcArr::V(Value::Number(v)) => {
                    print!("{:?}, ", v)
                }
                BcArr::V(Value::Reg(v)) => {
                    print!("{:?}, ", Value::Reg(v))
                }
                BcArr::V(Value::Pool(v)) => {
                    print!("{:?}, ", Value::Pool(v))
                }
                BcArr::V(Value::StringLiteral(v)) => {
                    print!("{:?}, ", v)
                }
                BcArr::V(Value::CPool(v)) => {
                    print!("{:?}, ", Value::CPool(v))
                }
                BcArr::V(Value::Bool(v)) => {
                    print!("{:?}, ", Value::Bool(v))
                }
                BcArr::V(Value::VAddr(v)) => {
                    print!("{:?}, ", Value::VAddr(v))
                }
                BcArr::V(Value::Nil) => {
                    print!("NIL")
                }
            }
        }
        if !program.const_pool.is_empty() {
            println!("\n+-----------Const-Pool-------------+\n");
            for (i, c) in program.const_pool.iter().enumerate() {
                println!("[{}] - {:?}", i, c);
            }
        }
        println!("\n+----------------------------------+\n");
    }

    let cfg = program.generate_cfg();
    //println!("CFG: {:#?}", cfg);

    let mut abstract_interpreter= AbstractInterpreter::new(&program);
    abstract_interpreter.run(&cfg[0].1);

    let mut vm = Interpreter::new(program);
    vm.interpret();
}
