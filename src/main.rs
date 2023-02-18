/**/

use anyhow::Context;
use clap::Parser;
use itertools::Itertools;
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
    let tokens = lex(&source, args.verbose)?;
    let instructions = parse(tokens, args.verbose)?;

    Ok(())
}

fn lex(source: &str, verbose: bool) -> Result<Vec<LabelPass1>, anyhow::Error> {
    let mut lex: logos::Lexer<Token> = Token::lexer(source);
    let mut definition_end = false;
    let mut tree: Vec<LabelPass1> = vec![];
    let mut prev_num = false;
    let mut current_token_tree: Vec<TokenTreePass1> = vec![];
    let mut defer_nest: Vec<Vec<TokenTreePass1>> = vec![];
    while let Some(token) = lex.next() {
        if verbose {
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
            current_token_tree.push(if prev_num {
                TokenTreePass1::Binary(x)
            } else {
                TokenTreePass1::Unary(y)
            });
            prev_num = false;
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

    if verbose {
        println!();
        println!();
        for LabelPass1(tokens) in tree.iter() {
            {
                println!("Label : ")
            };
            for i in tokens.iter() {
                println!("\t{i:?}")
            }
        }
    }
    Ok(tree)
}

fn parse(
    labels: Vec<LabelPass1>,
    verbose: bool,
) -> Result<Vec<Instruction>, anyhow::Error> {
    if verbose {
        println!();
        println!();
    }
    labels
        .into_iter()
        .filter(|LabelPass1(tokens)| !tokens.is_empty())
        .map(|LabelPass1(tokens)| parse_instruction(tokens, verbose))
        .collect()
}

fn parse_instruction(
    tokens: Vec<TokenTreePass1>,
    verbose: bool,
) -> Result<Instruction, anyhow::Error> {
    //if verbose {
    //    println!("{:?} =>", tokens);
    //}
    let mut tokens = tokens.into_iter().peekable();
    let label = match tokens.next() {
        Some(TokenTreePass1::Int(label)) => label,
        Some(_) => Err(Error::InvalidLabel)?,
        None => unreachable!(),
    };
    let mut intermediates = Vec::new();
    while let Some(separator) = tokens.next() {
        match separator {
            TokenTreePass1::Sep => (),
            other => Err(Error::ExpectedSeparatorInInsruction { got: other })?,
        }
        let tokens = tokens
            .by_ref()
            .peeking_take_while(|token| !is_separator(token));
        let expression = parse_expression(tokens, verbose)?;
        intermediates.push(expression);
    }
    let result = intermediates
        .pop()
        .ok_or_else(|| Error::ExpectedExpression)?;
    if verbose {
        println!("{}:", label);
        for intermediate in &intermediates {
            println!("\t\t{:?}", intermediate);
        }
        println!("\t{:?}", result);
    }
    let instruction = Instruction {
        label,
        intermediates,
        result,
    };
    Ok(instruction)
}

fn parse_expression(
    mut tokens: impl std::iter::Iterator<Item = TokenTreePass1>,
    verbose: bool,
) -> Result<Expression, anyhow::Error> {
    let mut expression = None;
    let mut stacked_unaries = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            TokenTreePass1::Unary(unary) => {
                stacked_unaries.push(unary);
            }
            TokenTreePass1::Binary(binary) => {
                let left = expression
                    .ok_or_else(|| Error::ExpectedExpressionBeforeBinary)?;
                let right = parse_expression(tokens, verbose)?;
                expression = Some(Expression::Binary {
                    operator: binary,
                    left: Box::new(left),
                    right: Box::new(right),
                });
                break;
            }
            _ if expression.is_some() => Err(Error::ExpectedOperator)?,
            TokenTreePass1::Int(integral) => {
                expression = Some(Expression::Int(integral));
            }
            TokenTreePass1::Float(float) => {
                expression = Some(Expression::Float(float));
            }
            TokenTreePass1::NestExpr(mut tokens) => {
                if tokens.is_empty() || tokens.iter().any(is_separator) {
                    let elements: Result<Vec<Expression>, anyhow::Error> =
                        tokens
                            .split_mut(is_separator)
                            .filter(|tokens| !tokens.is_empty())
                            .map(|tokens| {
                                // TODO avoid unnecessary clone here
                                let tokens: Vec<TokenTreePass1> =
                                    tokens.to_vec();
                                parse_expression(tokens.into_iter(), verbose)
                            })
                            .collect();
                    let elements = elements?;
                    expression = Some(Expression::List(elements));
                } else {
                    let inner = parse_expression(tokens.into_iter(), verbose)?;
                    expression = Some(inner);
                }
            }
            TokenTreePass1::Sep => Err(Error::ExpectedOperator)?,
        }
    }
    let mut expression = expression.ok_or_else(|| Error::ExpectedExpression)?;
    while let Some(unary) = stacked_unaries.pop() {
        expression = Expression::Unary {
            operator: unary,
            operand: Box::new(expression),
        };
    }
    Ok(expression)
}

fn is_separator(token: &TokenTreePass1) -> bool {
    match token {
        TokenTreePass1::Sep => true,
        _ => false,
    }
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

#[derive(Debug, Clone, Copy)]
enum Unary {
    Fetch,
    Signum,
    Neg,
    Recip,
}

#[derive(Debug, Clone, Copy)]
enum Binary {
    Plus,
    Mult,
    Assign,
    CallWith,
}

#[derive(Debug)]
struct LabelPass1(Vec<TokenTreePass1>);

#[derive(Debug, Clone)]
enum TokenTreePass1 {
    Int(Integral),
    Float(Float),
    NestExpr(Vec<TokenTreePass1>),
    Unary(Unary),
    Binary(Binary),
    Sep,
}

#[derive(Debug)]
struct Instruction {
    label: Integral,
    intermediates: Vec<Expression>,
    result: Expression,
}

#[derive(Debug)]
enum Expression {
    Int(Integral),
    Float(Float),
    List(Vec<Expression>),
    Unary {
        operator: Unary,
        operand: Box<Expression>,
    },
    Binary {
        operator: Binary,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Unbalanced delimiter")]
    UnbalancedDelimiter,
    #[error("Expected separator")]
    ExpectedSeparator,
    #[error("Expected separator, got {got:?}")]
    ExpectedSeparatorInInsruction { got: TokenTreePass1 },
    #[error("Expected expression")]
    ExpectedExpression,
    #[error("Expected expression before binary operator")]
    ExpectedExpressionBeforeBinary,
    #[error("Expected binary operator")]
    ExpectedOperator,
    #[error("Unstructured")]
    Unstructured,
    #[error("Invalid label, must be integral")]
    InvalidLabel,
}
