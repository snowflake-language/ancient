use num_bigint::BigInt;
use parser::ast::{Expression, OpSymbol, Statement, Tag, Type};
use std::{borrow::Cow, collections::HashMap};
use tag::{TagName, Universe, UniverseEntry, UniverseError};
use thiserror::Error;

// this is a hack, remove it
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EvaluatorConfig<'a> {
    pub project_tag: TagName<'a>,
    pub file_tags: HashMap<String, Vec<TagName<'a>>>,
}

pub struct Evaluator<'a> {
    universe: Universe<'a, UniverseItem>,
    config: EvaluatorConfig<'a>,
    pub entries: Vec<UniverseEntry<'a, UniverseItem>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedExpression(Type, Expression);

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UniverseItem {
    FnDecl {
        sig: Type,
        args: Vec<String>,
        body: Vec<Box<Expression>>,
    },

    // dummy variant used for implementing Default
    None,
}

impl Default for UniverseItem {
    fn default() -> Self {
        Self::None
    }
}

impl<'a> Evaluator<'a> {
    pub fn new(config: EvaluatorConfig<'a>) -> Self {
        Self {
            universe: Universe::default(),
            config,
            entries: Vec::new(),
        }
    }

    pub fn populate(
        &mut self,
        files: &HashMap<String, Vec<Statement>>,
    ) -> Result<(), FractalError> {
        // a mapping from a primary tag -> binding name -> tags + type + the body
        //
        // the options are required to handle the non-existance of a binding in the map
        let mut binding_cache: HashMap<
            TagName<'a>,
            HashMap<String, (Option<Vec<TagName<'a>>>, Option<Type>, Option<Statement>)>,
        > = HashMap::new();

        // insert the known primary tag
        binding_cache.insert(self.config.project_tag.clone(), HashMap::new());

        {
            // we currently ever handle one primary tag properly, just for my sanity <3
            // TODO(superwhiskers): make it handle multiple
            // TODO(superwhiskers): remove expect
            let cache = binding_cache
                .get_mut(&self.config.project_tag)
                .expect("unable to get a value that was just inserted into a map");

            // loop over the file list, accumulating bindings inside of the hashmap. this is done twice to
            // ensure that everything is captured
            for (file_path, contents) in files {
                // now, iterate over the contents of the file, sifting the bindings
                for stmt in contents {
                    // match against the statement, checking to see if it fits a set of accepted
                    // bindings
                    match stmt {
                        Statement::TypeDecl { name, body } => {
                            // since we have the name, we can now pull an entry out of the cache
                            let cache_entry = if let Some(k) = cache.get_mut(name) {
                                k
                            } else {
                                cache.insert(name.clone(), (None, None, None));

                                // TODO(superwhiskers): remove expect
                                cache.get_mut(name).expect(
                                    "unable to get a value that was just inserted into a map",
                                )
                            };

                            match body {
                                Type::FnSig { .. } => cache_entry.1 = Some(body.clone()),
                                Type::Tag(tag) => {
                                    // TODO(superwhiskers): remove expect
                                    let mut tags = self
                                        .config
                                        .file_tags
                                        .get(file_path)
                                        .expect(
                                            "unable to locate the current file in the file tag map",
                                        )
                                        .clone();

                                    tags.push(self.config.project_tag.clone());

                                    // flatten the Tag into an array of TagNames
                                    flatten_tag_opcall_to_tagnames(&mut tags, tag);

                                    // shove it into the cache entry
                                    cache_entry.0 = Some(tags);
                                }
                                _ => panic!("unexpected Type kind in type/tag signature"),
                            }
                        }
                        Statement::FnDecl { name, .. } => {
                            let cache_entry = if let Some(k) = cache.get_mut(name) {
                                k
                            } else {
                                cache.insert(name.clone(), (None, None, None));

                                // TODO(superwhiskers): remove expect
                                cache.get_mut(name).expect(
                                    "unable to get a value that was just inserted into a map",
                                )
                            };

                            cache_entry.2 = Some(stmt.clone());
                        }
                        _ => panic!("unexpected Statement kind at top level"),
                    }
                }
            }
        }

        // take the constructed HashMap and construct the Universe
        //
        // we don't care about taking ownership of the data, this HashMap isn't used past this
        // point
        for (_, primary_members) in binding_cache {
            for (binding_name, binding_value) in primary_members {
                // match over the Statement kind of it, as that's what the UniverseItem bases the
                // variant off of
                let (universe_item, tags) = match binding_value.2 {
                    Some(Statement::FnDecl { args, body, .. }) => {
                        if let Some(sig) = binding_value.1 {
                            if let Some(tags) = binding_value.0 {
                                (UniverseItem::FnDecl { sig, args, body }, tags)
                            } else {
                                panic!(
                                    "incomplete binding (missing tags) (this should never happen)"
                                );
                            }
                        } else {
                            panic!("incomplete binding (missing type)");
                        }
                    }
                    // TODO(superwhiskers): actually handle this properly
                    s => panic!("incomplete binding (missing valid statement, got {:?})", s),
                };

                self.entries.push(self.universe.insert(|b| {
                    b.set_name(Cow::Owned(binding_name))
                        .set_value(universe_item);
                    for tag in tags {
                        b.add_tag(tag);
                    }
                    b
                })?);
            }
        }

        Ok(())
    }

    // evaluate an expression and return the resulting expression
    pub fn eval_expression(
        &mut self,
        local_bindings: &mut HashMap<String, TypedExpression>,
        expr: &Box<Expression>,
    ) -> Result<Option<TypedExpression>, FractalError> {
        Ok(Some(match expr.as_ref() {
            Expression::Integer(int) => {
                TypedExpression(Type::Identifier(String::from("ilarge")), Expression::Integer(int.clone()))
            }
            Expression::StringLiteral(string) => TypedExpression(
                Type::Identifier(String::from("string")),
                Expression::StringLiteral(string.clone()),
            ),
            // if an identifier is passed all the way down, it is retrieved from the local bindings
            // hashamp
            //
            // TODO(superwhiskers): remove expect
            Expression::Identifier(ident) => local_bindings
                .get(ident)
                .expect("unable to retrieve the binding from locals")
                .clone(),
            Expression::FnCall {
                name,
                args,
            } => {
                match name.as_str() {
                    "println" => {
                        let boxed_arg = Box::new(args.get(0)
                                    .expect("unable to get the first argument to println").clone());
                        println!("{}", if args.len() == 0 {
                            String::from("")
                        } else {
                            // TODO(superwhiskers): remove expect
                            if let TypedExpression(
                                Type::Identifier(typen),
                                Expression::StringLiteral(string),
                            ) = self.eval_expression(
                                local_bindings,
                                &boxed_arg,
                            )?.expect("unable to evaluate to get a string") {
                                if typen != "string" {
                                    // TODO(superwhiskers): remove panic
                                    panic!("expression didn't return string to println");
                                }
                                string
                            } else {
                                // TODO(superwhiskers): remove panic
                                panic!("expression didn't return stringliteral to println");
                            }
                        });
                        return Ok(None);
                    }
                    _ => panic!("unknown function: {}", name),
                }
            }
            _ => panic!("invalid expression: {:?}", expr),
        }))
    }

    // evaluate a UniverseItem::FnDecl and return the resulting expression
    pub fn eval_fn(
        &mut self,
        item: UniverseItem,
        args: Vec<TypedExpression>,
    ) -> Result<Option<TypedExpression>, FractalError> {
        match item {
            UniverseItem::FnDecl {
                sig,
                args: arg_names,
                body,
            } => {
                // create a new binding set
                let mut bindings = HashMap::new();

                // TODO(superwhiskers): populate local bindings w/ intersected ones from universe

                // populate it with the arguments
                for i in 0..arg_names.len() {
                    bindings.insert(
                        arg_names
                            .get(i)
                            .expect("unable to index a vector at an existing indice")
                            .clone(),
                        args
                            .get(i)
                            .expect("missing argument at indice").clone(),
                    );
                }

                let mut last = Ok(None);
                for expr in &body {
                    last = self.eval_expression(&mut bindings, expr);
                }

                last
            }
            // TODO(superwhiskers): remove panic
            _ => panic!("not a function: {:?}", item),
        }
    }

    // evaluate a universe entry and return the resutling expression
    pub fn eval(
        &mut self,
        entry: &UniverseEntry<'a, UniverseItem>,
        args: Vec<TypedExpression>,
    ) -> Result<Option<TypedExpression>, FractalError> {
        // TODO(superwhiskers): remove expect
        self.eval_fn(
            self.universe
                .get(entry.binding)
                .expect("no binding found")
                .1
                .clone(),
            args,
        )
    }
}

/// helper recursive function used to flatten a tag OpCall into an array of TagNames
pub fn flatten_tag_opcall_to_tagnames<'a>(names: &mut Vec<TagName<'a>>, tag: &Tag) {
    match tag {
        Tag::OpCall { op, args } => {
            if *op != OpSymbol::Circumflex {
                // TODO(superwhiskers): remove panic
                panic!("operator is not `^`, it is {:?}", op);
            } else {
                for arg in args {
                    flatten_tag_opcall_to_tagnames(names, arg);
                }
            }
        }
        Tag::PrimaryIdentifier(name) => names.push(TagName::Primary(Cow::Owned(name.clone()))),
        Tag::Identifier(name) => names.push(TagName::Secondary(Cow::Owned(name.clone()))),
        _ => panic!("unexpected Tag kind {:?}", tag),
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum FractalError {
    #[error("An error was encountered while using the tag library")]
    UniverseError(#[from] UniverseError),
}
