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
    Dedent,
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
