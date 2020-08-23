use crate::indentation;
use crate::indentation::Indentation;
use crate::token::Token;
use logos::Logos;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
pub type Item = Spanned<Token, usize, String>;

fn spanned_token_into_item(span: (Token, logos::Span)) -> Item {
    let range = span.1;
    let token = span.0;
    Ok((range.start, token, range.end))
}

// todo: possibly change the way this works to be part of the lexing process itself
// todo: clean up dedent insertion code to not use Arc, and to use proper spans.
pub fn lex<'a>(source: &'a str) -> impl Iterator<Item = Item> + 'a {
    let mut indentation = indentation::IndentationLevel::new();
    let lexer = Token::lexer(source);

    lexer
        .spanned()
        // todo: find a possible way to do with without a box
        // convert Indentation tokens to usable Indent, Dedent, and Newline tokens.
        .flat_map(move |(tok, range)| match tok {
            Token::Indentation(level) => match indentation.update(level) {
                Ok(indent) => match indent {
                    Indentation::Indent => {
                        vec![(Token::Newline, range.clone()), (Token::Indent, range)]
                    }
                    Indentation::Dedent(count) => {
                        let mut tokens = vec![
                            (Token::Newline, range.clone()),
                            (Token::Dedent, range.clone()),
                        ];
                        if count > 1 {
                            for _ in 0..count {
                                tokens.push((Token::Dedent, range.clone()));
                            }
                        }
                        tokens
                    }
                    Indentation::Ondent => vec![(Token::Newline, range)],
                },
                Err(err) => vec![(Token::Error(String::from(err)), range)],
            },
            _ => vec![(tok, range)],
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
        "};

        let lexed: Vec<Token> = lex(input).map(|t| t.unwrap().1).collect();
        assert_eq!(
            lexed,
            vec![
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
                Token::Dedent,
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
            ]
        )
    }

    #[test]
    fn test_lex_dedent_insertion() {
        let input = indoc! {"
            a
                b
                    c
        "};

        let lexed: Vec<Token> = lex(input).map(|t| t.unwrap().1).collect();
        assert_eq!(
            lexed,
            vec![
                Token::Identifier(String::from("a")),
                Token::Newline,
                Token::Indent,
                Token::Identifier(String::from("b")),
                Token::Newline,
                Token::Indent,
                Token::Identifier(String::from("c")),
                Token::Newline,
                Token::Dedent,
                Token::Dedent,
                Token::Dedent,
            ]
        )
    }
}
