use std::env;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::process::exit;
use std::borrow::Cow;
use std::collections::HashMap;

use fractal::{Evaluator, EvaluatorConfig, UniverseItem};
use parser::ast::Statement;
use parser::lexer;
use parser::snowflake::ProgramParser;
use tag::{Universe, UniverseEntry, TagName};

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

    let mut universe = Universe::<UniverseItem>::default();

    let input = lexer::lex(&contents);
    let program = ProgramParser::new().parse(input).unwrap();

    let split: Vec<&str> = config.split(":").collect();
    let proj = split[0];
    let mut file_tags: HashMap<String, Vec<TagName>> = HashMap::new();

    let conf = EvaluatorConfig {
        project_tag: TagName::Primary(Cow::from(proj)),
        file_tags: file_tags,
    };
    let evaluator = Evaluator::new(conf);

    Ok(())
}
