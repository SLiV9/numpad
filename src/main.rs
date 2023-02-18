/**/

use anyhow::Context;
use clap::Parser;
use logos::Logos;

#[derive(Debug, clap::Parser)]
#[clap(version, propagate_version = true)]
struct Cli {
    /// One or more Penne source files
    #[clap(value_parser, required(true))]
    filepaths: Vec<std::path::PathBuf>,
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
    while let Some(t) = lex.next() {
        if t == Token::Error {
            println!("\t{:?}\t| {t:?} ", lex.slice())
        } else {
            print!("\n{:?}\t| {t:?} ", lex.slice().trim())
        }
        macro_rules! separator_warning {
            () => {
                "Unstructured, expected separator"
            };
        }
        use Token::*;
        let mut operator = |x, y| {
            if definition_end {
                panic!(separator_warning!())
            };
            prev_num = false;
            current_token_tree.push(if prev_num {
                TokenTreePass1::Binary(x)
            } else {
                TokenTreePass1::Unary(y)
            })
        };
        match t {
            Fetch_Mul => operator(Binary::Mult, Unary::Fetch),
            Signum_Plus => operator(Binary::Plus, Unary::Signum),
            Negate_Assign => operator(Binary::Assign, Unary::Neg),
            Reciprocal_CallWith => operator(Binary::CallWith, Unary::Recip),

            OpenExpr => {
                if prev_num || definition_end {
                    panic!(separator_warning!())
                };
                defer_nest.push(core::mem::take(&mut current_token_tree))
            }
            CloseExpr => {
                if definition_end {
                    panic!(separator_warning!())
                };
                let last = defer_nest.last_mut().expect("Unbalanced delimitor");
                last.push(TokenTreePass1::NestExpr(core::mem::take(
                    &mut current_token_tree,
                )));
                core::mem::swap(last, &mut current_token_tree);
                defer_nest.pop().unwrap();
            }
            Separator => {
                definition_end = false;
                prev_num = false;
                current_token_tree.push(TokenTreePass1::Sep)
            }
            Number => {
                if definition_end {
                    tree.push(LabelPass1(core::mem::take(
                        &mut current_token_tree,
                    )))
                };
                definition_end = false;
                prev_num = true;
                let src: String = lex.slice().split_whitespace().collect();
                current_token_tree.push(if src.contains(".") {
                    src.parse()
                        .map(TokenTreePass1::Float)
                        .expect("Logos Checked")
                } else {
                    src.parse().map(TokenTreePass1::Int).expect("Logos Checked")
                })
            }
            Enter => {
                definition_end = true;
            }
            Error if lex.slice().trim() == "" => {}
            Error => {
                if !lex.slice().starts_with('(') {
                    panic!("Unstructured")
                }
            } // )
        }
    }

    tree.push(LabelPass1(core::mem::take(&mut current_token_tree)));
    for LabelPass1(l) in tree.iter() {
        {
            println!("Label : ")
        };
        for i in l.iter() {
            println!("\t{i:?}")
        }
    }
    Ok(())
}

#[allow(non_camel_case_types)]
#[derive(Logos, Debug, PartialEq)]
enum Token {
    // unary operators
    #[regex(r"\*[ \t]*")]
    Fetch_Mul,
    #[regex(r"\+[ \t]*")]
    Signum_Plus,
    #[regex(r"\-[ \t]*")]
    Negate_Assign,
    #[regex(r"/[ \t]*")]
    Reciprocal_CallWith,
    #[regex(r"/\.[ \t]*")]
    OpenExpr,
    #[regex(r"\./[ \t]*")]
    CloseExpr,
    #[regex(r"\.\.[ \t]*")]
    Separator,
    #[regex(r"[0-9][0-9 \t]*(\.[0-9 \t]+)?")]
    Number,
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
