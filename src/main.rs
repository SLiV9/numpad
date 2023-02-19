/**/

mod common;
mod lexer;
mod machine;
mod parser;

use crate::common::*;
use crate::machine::Machine;

use clap::Parser;
use rustyline::{error::ReadlineError, DefaultEditor};

#[derive(Debug, clap::Parser)]
#[clap(version, propagate_version = true)]
struct Cli {
    /// One or more Numpad source files
    #[clap(value_parser)]
    filepaths: Vec<std::path::PathBuf>,

    /// Show a lot of intermediate output
    #[clap(short, long)]
    verbose: bool,

    /// Set the level of verbosity
    #[clap(long)]
    log_level: Option<log::Level>,

    /// Enable the REPL
    #[clap(short, long)]
    repl: bool,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    stderrlog::new()
        .module("numpad")
        .quiet(!args.verbose)
        .verbosity(args.log_level.unwrap_or(log::Level::Trace))
        .init()?;
    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let ref mut machine = Machine::create(vec![Instruction {
        label: 1,
        expression: Expression::Number(0.0),
    }]);
    let filepath = &args.filepaths.get(0);

    let repl = args.repl | filepath.is_none();

    if let Some(filepath) = filepath {
        let source = std::fs::read_to_string(&filepath)?;
        let tokens = lexer::lex(&source)?;
        let instructions = parser::parse(tokens)?;
        let output = evaluate(instructions, machine)?;
        println!("Output: {:?}", output);
    }

    if repl {
        let ref mut read = String::new();
        'exit: loop {
            // read
            read.clear();
            'read: loop {
                let mut readline = rl.readline("| ")?;
                rl.add_history_entry(readline.as_str())?;

                readline.push('\n');
                match readline.as_bytes() {
                    [b'0'..=b'9', ..] | [b'.', b'.', ..] => {}
                    [b'-', b'-', b'-', b'-', ..] => break 'exit,
                    [b'\n', ..] => break 'read,
                    _ => {
                        println!("Invalid starting character");
                        continue;
                    }
                }
                read.push_str(&readline)
            }
            // evaluate
            let tokens = match lexer::lex(&read) {
                Ok(t) => t,
                Err(e) => {
                    println!("Bad Input\nError :: {e}");
                    continue;
                }
            };
            let instructions = match parser::parse(tokens) {
                Ok(t) => t,
                Err(e) => {
                    println!("Bad Input\nError :: {e}");
                    continue;
                }
            };
            // print
            let output = evaluate(instructions, machine)?;
            println!("Output: {:?}", output);
            // loop
        }
    }

    rl.save_history("history.txt")?;
    Ok(())
}

fn evaluate(
    program: Vec<Instruction>,
    machine: &mut Machine,
) -> Result<Expression, anyhow::Error> {
    machine.update(program);
    let answer = machine.evaluate_until_finished(1);
    Ok(answer)
}
