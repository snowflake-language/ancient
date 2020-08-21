use parser::ast::{Expression, Type, OpSymbol, Statement};

fn eval_statement(statement: Statement) {
    match statement {
        Statement::FnDecl { name: n, args: a, body: b } => {
            
        },

        Statement::TypeDecl { name: n, body: b } => {
            match b {
                Type::FnSig { args: a, ret: r } => {
                    
                },
                Type::Nat(b) => {

                },
                Type::Identifier(s) => {

                },
            };
        },

        Statement::Expression(e) => {
            match e {
                Expression::OpCall { op: o, args: a } => {
                    match o {
                        OpSymbol::Plus => {

                        },
                        OpSymbol::Minus => {

                        },
                        OpSymbol::Star => {

                        },
                        OpSymbol::ForwardSlash => {

                        },
                    };
                },

                Expression::FnCall { name: n, args: a } => {

                },

                Expression::Integer(i) => {

                },

                Expression::Identifier(i) => {

                },
            };
        },
    };
}

pub fn eval(statement: Statement) {
    eval_statement(statement);
}
