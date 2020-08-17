#[macro_use]
extern crate lalrpop_util;
use logos::Logos;
pub mod ast;
pub mod indentation;
pub mod token;

use token::Token;

// maybe better name
lalrpop_mod!(pub crystaline);

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
pub type Item = Spanned<Token, usize, String>;

fn spanned_token_into_item(span: (Token, logos::Span)) -> Item {
    let range = span.1;
    let token = span.0;
    Ok((range.start, token, range.end))
}

pub fn parse<'a>(
    input: &'a str,
) -> Result<ast::Statement, lalrpop_util::ParseError<usize, token::Token, String>> {
    let input = token::Token::lexer(input)
        .spanned()
        .map(spanned_token_into_item);
    let mut indentation = indentation::IndentationLevel::new();
    // todo: make into ProgramParser
    crystaline::StatementParser::new().parse(&mut indentation, input)
}

#[cfg(test)]
mod test {
    use super::*;
    use ast;
    use ast::Expression;
    use ast::Statement;
    use crystaline::*;
    use num_bigint::BigInt;

    impl From<isize> for ast::Expression {
        fn from(i: isize) -> Self {
            ast::Expression::Integer(BigInt::from(i))
        }
    }

    impl<'a> From<&'a str> for ast::Expression {
        fn from(s: &'a str) -> Self {
            ast::Expression::Identifier(String::from(s))
        }
    }

    // test parse for
    macro_rules! test_parse {
        ($path:ty where $($input:expr => $test:expr),*) => {
            $({
                let input = token::Token::lexer($input).spanned().map(spanned_token_into_item);
                let mut indentation = indentation::IndentationLevel::new();
                let program = <$path>::new().parse(&mut indentation, input).unwrap();
                assert_eq!(program, $test)
            })*
        };
    }

    fn ops(l: impl Into<ast::Expression>, op: ast::OpSymbol, r: impl Into<ast::Expression>) -> ast::Expression {
        ast::Expression::OpCall {
            op: op,
            args: vec![Box::new(l.into()), Box::new(r.into())],
        }
    }

    #[test]
    fn parse_identifier() {
        test_parse! {
            IdentifierParser where
            "name" =>
                String::from("name"),
            "name_with_underscores_numbers_and_is_long_1234" =>
                String::from("name_with_underscores_numbers_and_is_long_1234")
        }
    }

    #[test]
    fn parse_integer() {
        test_parse! {
          IntegerParser where
          "132" => BigInt::from(132),
          "123_456_789" => BigInt::from(123_456_789)
        }
    }

    #[test]
    fn parse_literal() {
        test_parse! {
          LiteralParser where
          "132" => Expression::Integer(
            BigInt::from(132)
          ),
          "123_456_789" => Expression::Integer(
            BigInt::from(123_456_789)
          ),
          "name" => Expression::Identifier(
            String::from("name")
          ),
          "name_with_underscores_numbers_and_is_long_1234" => Expression::Identifier(
            String::from("name_with_underscores_numbers_and_is_long_1234")
          )
        }
    }

    #[test]
    fn parse_op_call() {
        test_parse! {
            OpCallParser where
            "1 + 1" => ast::Expression::OpCall {
                op: ast::OpSymbol::Plus,
                args: vec![
                    Box::new(1.into()),
                    Box::new(1.into()),
                ]
            },
            // should parse as
            // (1 + (2 * (3 - (4 / 5))))
            // although alternatively it could be made to parse like
            // ((((1 + 2) * 3) - 4) / 5)
            "1 + 2 * 3 - 4 / 5" => {
                use ast::OpSymbol::*;
                ops(1, Plus, ops(2, Star, ops(3, Minus, ops(4, ForwardSlash, 5))))
            }
        }
    }

    #[test]
    fn parse_block() {
        test_parse! {
            BlockParser where
            "\n  123\n  abc\n  123\n " => vec![
                Box::new(123.into()),
                Box::new("abc".into()),
                Box::new(123.into()),
            ]
        }
    }

    #[test]
    fn parse_fn_decl() {
        test_parse! {
            FnDeclParser where
            "add a b =>\n  a + b\n" => Statement::FnDecl {
                name: "add".into(),
                args: vec!["a".into(), "b".into()],
                body: vec![
                    Box::new(Expression::OpCall {
                        op: ast::OpSymbol::Plus,
                        args: vec![
                            Box::new("a".into()),
                            Box::new("b".into())
                        ]
                    })
                ]
            },
            "add a b => a + b" => Statement::FnDecl {
                name: String::from("add"),
                args: vec![
                    String::from("a"),
                    String::from("b")
                ],
                body: vec![
                    Box::new(Expression::OpCall {
                        op: ast::OpSymbol::Plus,
                        args: vec![
                            Box::new(Expression::from("a")),
                            Box::new(ast::Expression::from("b"))
                        ]
                    })
                ]
            }
        }
    }

    #[test]
    fn parse_fn_call() {
        test_parse! {
            FnCallParser where
            "add 1 2" => Expression::FnCall {
                name: "add".into(),
                args: vec![
                    1.into(),
                    2.into(),
                ]
            },
            "add a b" => Expression::FnCall {
                name: "add".into(),
                args: vec![
                    "a".into(),
                    "b".into(),
                ]
            }
        }
    }
}
