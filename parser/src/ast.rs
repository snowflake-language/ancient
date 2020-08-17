use num_bigint::BigInt;

// "top level" statements that are not an expression
// while anything can be a statement, I think a goal should be that
// anything can return a value
// - @bree
#[derive(Debug, PartialEq)]
pub enum Statement {
    FnDecl {
        name: String,
        args: Vec<String>,
        body: Vec<Box<Expression>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    OpCall { op: OpSymbol, args: Vec<Box<Expression>> },
    FnCall { name: String, args: Vec<Expression> },
    Integer(BigInt),
    Identifier(String),
}

// named OpSymbol so it has some "genericness" for future use
// in something like macros
// - @bree
#[derive(Debug, PartialEq)]
pub enum OpSymbol {
    Plus,
    Minus,
    Star,
    ForwardSlash,
}
