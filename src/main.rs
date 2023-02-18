/**/

use anyhow::Context;
use clap::Parser;
use logos::Logos;

#[derive(Debug, clap::Parser)]
#[clap(version, propagate_version = true)]
struct Cli {
    /// One or more Numpad source files
    #[clap(value_parser, required(true))]
    filepaths: Vec<std::path::PathBuf>,

    /// Show a lot of intermediate output
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    let filepath = &args.filepaths.get(0).context("missing argument")?;
    let source = std::fs::read_to_string(&filepath)?;

    let mut lex: logos::Lexer<Token> = Token::lexer(&source);
    let mut definition_end = false;
    let mut tree: Vec<LabelPass1> = vec![];
    let mut prev_num = false;
    let mut current_token_tree: Vec<TokenTreePass1> = vec![];
    let mut defer_nest: Vec<Vec<TokenTreePass1>> = vec![];
    while let Some(token) = lex.next() {
        if args.verbose {
            if token == Token::Error {
                println!("\n{:?}\t| {token:?} ", lex.slice())
            } else {
                print!("\n{:?}\t| {token:?} ", lex.slice().trim())
            }
        }
        let mut operator = |x, y| -> Result<(), anyhow::Error> {
            if definition_end {
                Err(Error::ExpectedSeparator)?;
            };
            prev_num = false;
            current_token_tree.push(if prev_num {
                TokenTreePass1::Binary(x)
            } else {
                TokenTreePass1::Unary(y)
            });
            Ok(())
        };
        match token {
            Token::Star => operator(Binary::Mult, Unary::Fetch)?,
            Token::Plus => operator(Binary::Plus, Unary::Signum)?,
            Token::Minus => operator(Binary::Assign, Unary::Neg)?,
            Token::Slash => operator(Binary::CallWith, Unary::Recip)?,

            Token::OpenExpr => {
                if prev_num || definition_end {
                    Err(Error::ExpectedSeparator)?;
                };
                defer_nest.push(core::mem::take(&mut current_token_tree))
            }
            Token::CloseExpr => {
                if definition_end {
                    Err(Error::ExpectedSeparator)?;
                };
                let last = defer_nest
                    .last_mut()
                    .ok_or_else(|| Error::UnbalancedDelimiter)?;
                last.push(TokenTreePass1::NestExpr(core::mem::take(
                    &mut current_token_tree,
                )));
                core::mem::swap(last, &mut current_token_tree);
                defer_nest.pop().unwrap();
            }
            Token::Separator => {
                definition_end = false;
                prev_num = false;
                current_token_tree.push(TokenTreePass1::Sep)
            }
            Token::Number => {
                if definition_end {
                    tree.push(LabelPass1(core::mem::take(
                        &mut current_token_tree,
                    )))
                };
                definition_end = false;
                prev_num = true;
                let src: String = lex.slice().split_whitespace().collect();
                current_token_tree.push(if src.contains(".") {
                    src.parse().map(TokenTreePass1::Float)?
                } else {
                    src.parse().map(TokenTreePass1::Int)?
                })
            }
            Token::Enter => {
                definition_end = true;
            }
            Token::Error if lex.slice().trim() == "" => {}
            Token::Error if lex.slice().starts_with('(') => {}
            Token::Error => Err(Error::Unstructured)?,
        }
    }
    tree.push(LabelPass1(core::mem::take(&mut current_token_tree)));

    if args.verbose {
        println!();
        println!();
        for LabelPass1(l) in tree.iter() {
            {
                println!("Label : ")
            };
            for i in l.iter() {
                println!("\t{i:?}")
            }
        }
    }
    Ok(())
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // Operators
    #[regex(r"\*[ \t]*")]
    Star,
    #[regex(r"\+[ \t]*")]
    Plus,
    #[regex(r"\-[ \t]*")]
    Minus,
    #[regex(r"/[ \t]*")]
    Slash,

    // Structurals
    #[regex(r"/\.[ \t]*")]
    OpenExpr,
    #[regex(r"\./[ \t]*")]
    CloseExpr,
    #[regex(r"\.\.[ \t]*")]
    Separator,

    // Literals
    #[regex(r"[0-9][0-9 \t]*(\.[0-9 \t]+)?")]
    Number,

    // Display
    #[token("\n")]
    Enter,
    #[error]
    #[regex(r"\(.*\)", logos::skip)]
    Error,
}

type Integral = usize;

type Float = f64;

#[derive(Debug)]
enum Unary {
    Fetch,
    Signum,
    Neg,
    Recip,
}

#[derive(Debug)]
enum Binary {
    Plus,
    Mult,
    Assign,
    CallWith,
}

#[derive(Debug)]
struct LabelPass1(Vec<TokenTreePass1>);

#[derive(Debug)]
enum TokenTreePass1 {
    Int(Integral),
    Float(Float),
    NestExpr(Vec<TokenTreePass1>),
    Unary(Unary),
    Binary(Binary),
    Sep,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Unbalanced delimiter")]
    UnbalancedDelimiter,
    #[error("Unstructured, expected separator")]
    ExpectedSeparator,
    #[error("Unstructured")]
    Unstructured,
}
