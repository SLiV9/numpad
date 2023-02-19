/**/

mod common;
mod lexer;
mod machine;
mod parser;

use crate::common::*;
use crate::machine::Machine;

use clap::Parser;

#[derive(Debug, clap::Parser)]
#[clap(version, propagate_version = true)]
struct Cli {
    /// One or more Numpad source files
    #[clap(value_parser)]
    filepaths: Vec<std::path::PathBuf>,

    /// Show a lot of intermediate output
    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    repl: bool
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    let ref mut machine = Machine::create(
        vec![Instruction{label:1,expression:Expression::Number(0.0)}], 
        args.verbose
    );
    let filepath = &args.filepaths.get(0);

    let repl = args.repl | filepath.is_none();
    
    if let Some(filepath) = filepath {
        let source = std::fs::read_to_string(&filepath)?;
        let tokens = lexer::lex(&source, args.verbose)?;
        let instructions = parser::parse(tokens, args.verbose)?;
        let output = evaluate(instructions, machine)?;
        println!("Output: {:?}", output);
    }
    //   let output = evaluate(vec![], machine)?;
    //   println!("Output: {:?}", output);


    if repl {
        
        let ref mut read = String::new();
        let ref mut last_line = String::new();
        'exit : loop {
            // read
            read.clear();
            'read : loop {
                last_line.clear();
                std::io::stdin().read_line(last_line)?;
                last_line.push('\n');
                match last_line.as_bytes() {
                    [b'0'..=b'9', ..]
                    | [b'.',b'.',..]  => {}
                    [b'-',b'-',b'-',b'-',..] => break 'exit ,
                    [b'\n', ..]      => {break 'read}
                    _ => {println!("Invalid starting character"); continue}
                }
                read.push_str(last_line)
            }
            // evaluate
            let tokens = match lexer::lex(&read, args.verbose) {
              Ok(t) => t,
              Err(e) => {println!("Bad Input\nError :: {e}"); continue}
            };
            let instructions = match parser::parse(tokens, args.verbose) {
              Ok(t) => t,
              Err(e) => {println!("Bad Input\nError :: {e}"); continue}
            };
            // print
            let output = evaluate(instructions, machine)?;
            println!("Output: {:?}", output);
            // loop
        }
    }


    Ok(())
}

fn evaluate(
    program: Vec<Instruction>,
    machine: &mut Machine
) -> Result<Expression, anyhow::Error> {
    machine.update(program);
    let answer = machine.evaluate_until_finished(1);
    Ok(answer)
}
