use std::env;
use std::fs::File;
use std::io::{Read};
use std::process::exit;
use std::borrow::Cow;
use std::collections::HashMap;

use fractal::{Evaluator, EvaluatorConfig, TypedExpression};
use parser::{lexer, snowflake::ProgramParser, ast::{Statement, Type, Expression}};
use tag::{TagName};

// Wrapper for unwrapping Results and printing errors cleanly
macro_rules! unwrap {
    ( $x:expr ) => {
        match $x {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Error!: {}", e);
                exit(1);
            }
        }
    };
    ( debug $x:expr ) => {
        match $x {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Error!: {:?}", e);
                exit(1);
            }
        }
    };
    ( pretty $x:expr ) => {
        match $x {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Error!: {:#?}", e);
                exit(1);
            }
        }
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: flurry [FILE] [CONFIG]");
        exit(1);
    }

    let mut file = unwrap!(File::open(&args[1]));
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut config_file = unwrap!(File::open(&args[2]));
    let mut config = String::new();
    config_file.read_to_string(&mut config)?;

    let input = lexer::lex(&contents);
    let program = ProgramParser::new().parse(input).unwrap();

    let split: Vec<&str> = config.split(":").collect();
    let proj = split[0];
    let mut file_tags: HashMap<String, Vec<TagName>> = HashMap::new();
    file_tags.insert(args[2].clone(), Vec::new()); // required for evaluator.prepare to work

    let conf = EvaluatorConfig {
        project_tag: TagName::Primary(Cow::from(proj)),
        file_tags,
    };
    let mut evaluator = Evaluator::new(conf);

    let mut source: HashMap<String, Vec<Statement>> = HashMap::new();
    source.insert(args[2].clone(), program);
    evaluator.populate(&source)?;

    let main = evaluator.entries
        .iter_mut()
        .filter(|t| t.binding.index() == 0)
        .next()
        .unwrap()
        .clone();

    evaluator.eval(&main, vec![TypedExpression(Type::Identifier(String::from("ilarge")), Expression::Integer(69.into()))])?;

    Ok(())
}
