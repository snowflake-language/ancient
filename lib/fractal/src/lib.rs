use num_bigint::BigInt;
use parser::ast::{Expression, OpSymbol, Statement, Tag, Type};
use std::{borrow::Cow, collections::HashMap};
use tag::{TagName, Universe, UniverseError, UniverseEntry};
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
    universe_entries: Vec<UniverseEntry<'a, UniverseItem>>,
}

#[derive(Debug, PartialEq, Clone)]
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
    pub fn prepare(&mut self, files: &HashMap<String, Vec<Statement>>) -> Result<(), FractalError> {
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
                    s => panic!(format!("incomplete binding (missing valid statement, got {:?})", s)),
                };

                self.universe_entries.push(self.universe.insert(|b| {
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

    /*
    pub fn eval<'a>(
        &mut self
        : Statement,
    ) -> Result<(), FractalError> {
        Ok(())
    }
    */
}

/// helper recursive function used to flatten a tag OpCall into an array of TagNames
pub fn flatten_tag_opcall_to_tagnames<'a>(names: &mut Vec<TagName<'a>>, tag: &Tag) {
    match tag {
        Tag::OpCall { op, args } => {
            if *op != OpSymbol::Circumflex {
                // TODO(superwhiskers): remove panic
                panic!(format!("operator is not `^`, it is {:?}", op));
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
