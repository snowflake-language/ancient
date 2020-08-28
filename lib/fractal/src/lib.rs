use std::borrow::Cow;
use std::process::exit;

use parser::ast::{Expression, OpSymbol, Statement, Type};
use tag::{Binding, BindingBuilder, Universe, UniverseBuilder, UniverseError};

fn eval_statement<'a>(
    universe: &mut Universe<'a, Statement>,
    statement: Statement,
) -> Result<(), Box<dyn std::error::Error>> {
    match statement {
        Statement::FnDecl {
            name: n,
            args: a,
            body: b,
        } => {
            // TODO(@monarrk): This could really be cleaner
            let r = universe.insert(|x| {
                x.set_name(Cow::from(n.clone()))
                    .set_value(Statement::FnDecl {
                        name: n.clone(),
                        args: a.clone(),
                        body: b.clone(),
                    })
            });

            return match r {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            };
        }

        Statement::TypeDecl { name: n, body: b } => {
            match b {
                Type::FnSig { args: a, ret: r } => {}
                Type::Nat(b) => {}
                Type::Identifier(s) => {}
            };
        }

        Statement::Expression(e) => {
            match e {
                Expression::OpCall { op: o, args: a } => {
                    match o {
                        OpSymbol::Plus => {}
                        OpSymbol::Minus => {}
                        OpSymbol::Star => {}
                        OpSymbol::ForwardSlash => {}
                    };
                }

                Expression::FnCall { name: n, args: a } => {}

                Expression::Integer(i) => {}

                Expression::Identifier(i) => {}
            };
        }
    };

    Ok(())
}

/// Execute a snowflake AST
pub fn eval(statement: Statement) -> Result<(), Box<dyn std::error::Error>> {
    let mut universe = Universe::<Statement>::default();
    eval_statement(&mut universe, statement)
}

#[cfg(test)]
mod test {
    use super::*;
    use parser::lexer;
    use parser::snowflake::*;

    #[test]
    fn eval_fn_decl() {
        eval(
            FnDeclParser::new()
                .parse(lexer::lex("add a b =>\n  a + b\n"))
                .unwrap(),
        )
        .unwrap();
    }
}
