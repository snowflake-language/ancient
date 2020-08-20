//
// parser - snowflake's parser
//
// copyright (c) 2020 the snowflake authors <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use logos::Logos;
use num_bigint::BigInt;
use crate::indentation;
use crate::indentation::Indentation;

fn lex_char(lex: &mut logos::Lexer<Token>) -> Option<char> {
    lex.source().chars().nth(lex.span().start)
}

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
    #[regex("[a-zA-Z][a-zA-Z1-9_]*", |lex| lex.slice().parse())]
    Identifier(String),

    #[regex(r"[0-9]*\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    #[regex("[0-9][0-9_]*", |lex| lex.slice().parse())]
    Integer(BigInt),

    // replaced with inserted tokens
    #[regex("\n(  )*", |lex| ((lex.slice().len() - 1) / 2))]
    Indentation(usize),

    // todo: eventually give proper names to some of these
    // "non-symbol" character combinations
    #[token("=")]
    Equal,

    #[token("::")]
    ColonColon,

    #[token("**")]
    StarStar,

    #[token("=>")]
    LargeArrowRight,

    #[token("->")]
    SmallArrowRight,

    #[regex(r"\s", logos::skip)]
    Whitespace,

    #[regex(r"[!-/:-@\[-`{-~()]", lex_char)]
    Symbol(char),

    #[error(|lex| lex.slice().parse())]
    Unknown,

    // inserted tokens
    Newline,
    Indent,
    Dedent
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use Token::*;

    #[test]
    fn lex_test() {
        let source = indoc! {"
            fib :: isize -> isize
            fib n =>
              (fib n - 1) + (fib n - 2)
        "};
        let tokens: Vec<_> = Token::lexer(source).collect();
        assert_eq!(
            tokens,
            vec![
                Identifier(String::from("fib")),
                ColonColon,
                Identifier(String::from("isize")),
                SmallArrowRight,
                Identifier(String::from("isize")),
                Indentation(0),
                Identifier(String::from("fib")),
                Identifier(String::from("n")),
                LargeArrowRight,
                Indentation(1),
                Symbol('('),
                Identifier(String::from("fib")),
                Identifier(String::from("n")),
                Symbol('-'),
                Integer(BigInt::from(1)),
                Symbol(')'),
                Symbol('+'),
                Symbol('('),
                Identifier(String::from("fib")),
                Identifier(String::from("n")),
                Symbol('-'),
                Integer(BigInt::from(2)),
                Symbol(')'),
                // the final newline is from indoc
                Indentation(0)
            ]
        )
    }

    #[test]
    fn lex_indent_test() {
        let source = indoc! {"
            block =>
              123
              abc
              123
        "};
        let tokens: Vec<_> = Token::lexer(source).collect();
        assert_eq!(
            tokens,
            vec![
                Identifier(String::from("block")),
                LargeArrowRight,
                Indentation(1),
                Integer(BigInt::from(123)),
                Indentation(1),
                Identifier(String::from("abc")),
                Indentation(1),
                Integer(BigInt::from(123)),
                Indentation(0),
            ]
        )
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
pub type Item = Spanned<Token, usize, String>;

fn spanned_token_into_item(span: (Token, logos::Span)) -> Item {
    let range = span.1;
    let token = span.0;
    Ok((range.start, token, range.end))
}

// todo: possibly change the way this works to be part of the lexing process itself
// todo: insert dedent at end of input
pub fn lex<'a>(source: &'a str) -> impl Iterator<Item=Item> + 'a {
    let mut indentation = indentation::IndentationLevel::new();
    let lexer = Token::lexer(source);

    lexer
        .spanned()
        .flat_map(move |(tok, range)| {
            match tok {
                Token::Indentation(level) => {
                    match indentation.update(level) {
                        Indentation::Indent => vec![(Token::Newline, range.clone()), (Token::Indent, range)],
                        Indentation::Dedent => vec![(Token::Newline, range.clone()), (Token::Dedent, range)],
                        Indentation::Ondent => vec![(Token::Newline, range)]
                    }
                },
                _ => vec![(tok, range)]
            }
        })
        .map(spanned_token_into_item)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_lex() {
        let input = indoc! {"
            a
                b
                    c
                d
                    e
            f
            g
                    h
                i
            j
        "};

        let lexed: Vec<Token> = lex(input).map(|t|t.unwrap().1).collect();
        assert_eq!(lexed, vec![
            Token::Identifier(String::from("a")),
            Token::Newline,
            Token::Indent,
            Token::Identifier(String::from("b")),
            Token::Newline,
            Token::Indent,
            Token::Identifier(String::from("c")),
            Token::Newline,
            Token::Dedent,
            Token::Identifier(String::from("d")),
            Token::Newline,
            Token::Indent,
            Token::Identifier(String::from("e")),
            Token::Newline,
            Token::Dedent,
            Token::Identifier(String::from("f")),
            Token::Newline,
            Token::Identifier(String::from("g")),
            Token::Newline,
            Token::Indent,
            Token::Identifier(String::from("h")),
            Token::Newline,
            Token::Dedent,
            Token::Identifier(String::from("i")),
            Token::Newline,
            Token::Dedent,
            Token::Identifier(String::from("j")),
            Token::Newline
        ])
    }
}