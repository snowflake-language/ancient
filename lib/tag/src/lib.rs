//
// tag - snowflake's tagging library backend
//
// copyright (c) 2020 the snowflake authors <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//! The library behind the tag system that backs snowflake
//!
//! # What is a tag?
//!
//! A tag is, in essence, a [mathematical set] with a few changes. The most important of these
//! changes is that there are two kinds: _primary tags_ and _secondary tags_.
//!
//! A _primary tag_ is equivalent to a mathematical sets in all regards, with one exception
//! documented below. A _secondary tag_ is the same but without a uniqueness restriction.
//!
//! There is one more change, and that is that due to how tags work (they're collections of
//! bindings from a name to a value), the uniqueness restriction is only enforced on the name of
//! the binding, and not at all on the value
//!
//! # I don't understand this. Can you explain it a little simpler?
//!
//! If you've ever used a [venn diagram], they can be visualized in the same way. Tag intersections
//! are where the components overlap, unions are any two components combined, symmetric difference
//! (xor) is everything that doesn't overlap, and so on...
//!
//! # How do I use this?
//!
//! todo
//!
//! [mathematical set]: https://en.wikipedia.org/wiki/Set_theory
//! [venn diagram]: https://en.wikipedia.org/wiki/Venn_diagram

#![allow(clippy::cognitive_complexity)]
#![warn(clippy::cargo_common_metadata)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::explicit_deref_methods)]
#![warn(clippy::filetype_is_file)]
#![warn(clippy::imprecise_flops)]
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::cast_lossless)]
#![deny(clippy::clone_on_ref_ptr)]
#![deny(clippy::doc_markdown)]
#![deny(clippy::empty_enum)]
#![deny(clippy::enum_glob_use)]
#![deny(clippy::exit)]
#![deny(clippy::explicit_into_iter_loop)]
#![deny(clippy::explicit_iter_loop)]
#![deny(clippy::fallible_impl_from)]
#![deny(clippy::inefficient_to_string)]
#![deny(clippy::large_digit_groups)]
#![deny(clippy::wildcard_dependencies)]
#![deny(clippy::wildcard_imports)]
#![deny(clippy::unused_self)]
#![deny(clippy::single_match_else)]
#![deny(clippy::option_option)]
#![deny(clippy::mut_mut)]
#![feature(bool_to_option)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(concat_idents)]
#![feature(or_patterns)]

use id_arena::{Arena, ArenaBehavior, DefaultArenaBehavior};
use sdset::{duo::OpBuilder, Error as SdsetError, Set, SetOperation};
use std::{borrow::Cow, clone::Clone, collections::hash_map::HashMap};
use thiserror::Error;

/// A builder-like type, used in construction of a [`Binding`]
///
/// [`Binding`]: ./struct.Binding.html
#[derive(Debug, Default)]
pub struct BindingBuilder<'a, T>
where
    T: Default + Clone,
{
    pub(crate) binding: Binding<'a, T>,
    pub(crate) tags: Vec<TagName<'a>>,
}

impl<'a, T> BindingBuilder<'a, T>
where
    T: Default + Clone,
{
    /// Sets the name portion of the [`Binding`]
    ///
    /// [`Binding`]: ./struct.Binding.html
    pub fn set_name(&mut self, name: Cow<'a, str>) -> &mut Self {
        self.binding.name = name;
        self
    }

    /// Sets the value portion of the [`Binding`]
    ///
    /// [`Binding`]: ./struct.Binding.html
    pub fn set_value(&mut self, value: T) -> &mut Self {
        self.binding.value = value;
        self
    }

    /// Adds the [`Binding`] to a [`Tag`] by its [`TagName`]
    ///
    /// [`Binding`]: ./struct.Binding.html
    /// [`Tag`]: ./enum.Tag.html
    /// [`TagName`]: ./enum.TagName.html
    pub fn add_tag(&mut self, tag: TagName<'a>) -> &mut Self {
        self.tags.push(tag);
        self
    }

    /// Removes the provided [`TagName`] from the [`Binding`]'s current [`Tag`]s
    ///
    /// [`TagName`]: ./enum.TagName.html
    /// [`Binding`]: ./struct.Binding.html
    /// [`Tag`]: ./enum.Tag.html
    pub fn remove_tag(mut self, tag: TagName<'a>) -> Self {
        self.tags = self.tags.into_iter().filter(|t| *t != tag).collect();
        self
    }
}

/// A relation between a string and its corresponding value. The string is considered to be the
/// uniqueness specifier, the value has no play in equality
#[derive(Debug, Default, Clone)]
pub struct Binding<'a, T>
where
    T: Default + Clone,
{
    pub name: Cow<'a, str>,
    pub value: T,
}

impl<'a, T> PartialEq for Binding<'a, T>
where
    T: Default + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a, T: Default + Clone> Eq for Binding<'a, T> {}

/// The state of a group of [`Tag`]s. e.g. is it composed of either [`Tag::Primary`]s or
/// [`Tag::Secondary`], or rather a mix of the two instead?
///
/// [`Tag`]: ./enum.Tag.html
/// [`Tag::Primary`]: ./enum.Tag.html#variant.Primary
/// [`Tag::Secondary`]: ./enum.Tag.html#variant.Secondary
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TagGroupComposition {
    Primary,
    PrimaryAndSecondary,
    Secondary,
    SecondaryAndPrimary,
}

/// A tag's name. Used to key names to the corresponding tags in a [`HashMap`] without special
/// syntax (i.e. `*`, which is used to denote a primary one in snowflake itself)
///
/// [`HashMap`]: https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum TagName<'a> {
    Primary(Cow<'a, str>),
    Secondary(Cow<'a, str>),
}

/// A tag. Primary tags are equivalent to mathematical sets, secondary tags are the same but
/// without the uniqueness restriction. Within the contained [`Binding`], only the [`name`] must be
/// unique
///
/// Note: there is no validation done on the names of the contained bindings; the task of ensuring
/// name validity is up to you
///
/// [`Binding`]: ./struct.Binding.html
/// [`name`]: ./struct.Binding.html#structfield.name
#[derive(Debug, PartialEq, Eq)]
pub enum Tag<'a, T>
where
    T: Default + Clone,
{
    Primary(Vec<<DefaultArenaBehavior<Binding<'a, T>> as ArenaBehavior>::Id>),
    Secondary(Vec<<DefaultArenaBehavior<Binding<'a, T>> as ArenaBehavior>::Id>),
}

impl<'a, T> Tag<'a, T>
where
    T: Default + Clone,
{
    /// Retrieves a reference to the inner vector of the [`Tag`] as a [`Set`]. If the vector is not
    /// sorted, then it will fail. Because of this, it is recommended to call [`Tag::sort`] before
    /// calling this method
    ///
    /// [`Tag`]: ./enum.Tag.html
    /// [`Set`]: https://docs.rs/sdset/0.4.0/sdset/set/struct.Set.html
    /// [`Tag::sort`]: ./enum.Tag.html#method.sort
    pub fn as_set(
        &self,
    ) -> Result<&Set<<DefaultArenaBehavior<Binding<'a, T>> as ArenaBehavior>::Id>, SdsetError> {
        Set::new(self.as_slice())
    }

    /// Retrieves a reference to the inner vector of the [`Tag`]
    ///
    /// [`Tag`]: ./enum.Tag.html
    pub fn as_slice(&self) -> &[<DefaultArenaBehavior<Binding<'a, T>> as ArenaBehavior>::Id] {
        match self {
            Tag::Primary(s) => s,
            Tag::Secondary(s) => s,
        }
    }

    /// Retrieves a mutable reference to the inner vector of the [`Tag`]
    ///
    /// [`Tag`]: ./enum.Tag.html
    pub fn as_mut_slice(
        &mut self,
    ) -> &mut [<DefaultArenaBehavior<Binding<'a, T>> as ArenaBehavior>::Id] {
        match self {
            Tag::Primary(s) => s,
            Tag::Secondary(s) => s,
        }
    }

    /// Sorts the [`Tag`]'s contents. Primarily useful before calling [`Tag::set`]
    ///
    /// [`Tag`]: ./enum.Tag.html
    /// [`Tag::set`]: ./enum.Tag.html#method.set
    pub fn sort(&mut self) {
        self.as_mut_slice().sort();
    }
}

/// An enumeration over possible operations for a [`UniverseOperationBuilder`] to use
///
/// [`UniverseOperationBuilder`]: ./struct.UniverseOperationBuilder.html
#[derive(Debug)]
pub enum UniverseOperationOp {
    Union,
    Intersection,
    Difference,
    SymmetricDifference,
}

/// A builder type for a [`UniverseOperationOp`]
///
/// [`UniverseOperationOp`]: ./struct.UniverseOperationOp.html
#[derive(Debug, Default)]
pub struct UniverseOperationBuilder<'a> {
    tag_names: Option<(TagName<'a>, TagName<'a>)>,
    op: Option<UniverseOperationOp>,
}

impl<'a> UniverseOperationBuilder<'a> {
    pub fn sets(&mut self, tags: (TagName<'a>, TagName<'a>)) -> &mut Self {
        self.tag_names = Some(tags);
        self
    }

    pub fn set_operation(&mut self, op: UniverseOperationOp) -> &mut Self {
        self.op = Some(op);
        self
    }
}

/// A builder-like type, used to ease in the creation of the [`Universe`] type
///
/// [`Universe`]: ./struct.Universe.html
#[derive(Debug, Default)]
pub struct UniverseBuilder {
    pub(crate) tag_hashmap_capacity: Option<usize>,
    pub(crate) binding_arena_capacity: Option<usize>,
}

impl UniverseBuilder {
    /// "Builds" the builder type, returning a [`Universe`]
    ///
    /// [`Universe`]: ./struct.Universe.html
    fn build<'a, T>(&mut self) -> Universe<'a, T>
    where
        T: Default + Clone,
    {
        Universe {
            bindings: self
                .binding_arena_capacity
                .map_or_else(|| Arena::new(), |capacity| Arena::with_capacity(capacity)),
            tags: self.tag_hashmap_capacity.map_or_else(
                || HashMap::new(),
                |capacity| HashMap::with_capacity(capacity),
            ),
        }
    }

    /// Sets the number of elements to reserve capacity for in the tag [`HashMap`]
    ///
    /// [`HashMap`]: https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html
    pub fn with_tag_hashmap_capacity(&mut self, capacity: usize) -> &mut Self {
        self.tag_hashmap_capacity = Some(capacity);
        self
    }

    /// Sets the number of elements to reserve capacity for in the binding [`Arena`]
    ///
    /// [`Arena`]: https://docs.rs/id-arena/2.2.1/id_arena/struct.Arena.html
    pub fn with_binding_arena_capacity(&mut self, capacity: usize) -> &mut Self {
        self.binding_arena_capacity = Some(capacity);
        self
    }
}

/// A reference to an entry in a [`Universe`]
///
/// [`Universe`]: ./struct.Universe.html
pub struct UniverseEntry<'a, T>
where
    T: Default + Clone,
{
    pub binding: <DefaultArenaBehavior<Binding<'a, T>> as ArenaBehavior>::Id,
    pub tags: Vec<TagName<'a>>,
}

impl<'a, T> UniverseEntry<'a, T>
where
    T: Default + Clone,
{
    // /// Convert the [`UniverseEntry`] to an owned
}

/// A list of possible errors that may be encountered while working with a [`Universe`]
///
/// [`Universe`]: ./struct.Universe.html
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum UniverseError {
    /// An error returned if such a binding already exists inside of the [`Universe`]
    ///
    /// Note: this does not contain a [`UniverseEntry`] of the existing binding due to the effort
    /// required to generate one
    ///
    /// [`Universe`]: ./struct.Universe.html
    #[error("The provided binding name is already in use")]
    BindingAlreadyExists,

    /// An error returned if no [`Tag`] corresponds to the provided [`TagName`]
    ///
    /// [`Tag`]: ./enum.Tag.html
    /// [`TagName`]: ./enum.TagName.html
    #[error("The provided TagName has no corresponding Tag")]
    InvalidTagName,

    /// An error returned if no [`Tag`]s were provided to a [`UniverseOperationBuilder`]
    ///
    /// [`Tag`]: ./enum.Tag.html
    /// [`UniverseOperationBuilder`]: ./struct.UniverseOperationBuilder.html
    #[error("Not enough Tags were provided")]
    NoTagsProvided,

    /// An error returned if no [`UniverseOperationOp`] was provided to a
    /// [`UniverseOperationBuilder`]
    ///
    /// [`UniverseOperationOp`]: ./enum.UniverseOperationOp.html
    /// [`UniverseOperationBuilder`]: ./struct.UniverseOperationBuilder.html
    #[error("No UniverseOperationOp was provided")]
    NoOperationProvided,

    /// An error that may be encountered while working with the sdset library
    #[error("An error was encountered while using the `sdset` library")]
    SdsetError(#[from] SdsetError),
}

/// A collection of [`Tag`]s and their [`Binding`]s
///
/// [`Tag`]: ./enum.Tag.html
/// [`Binding`]: ./struct.Binding.html
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Universe<'a, T>
where
    T: Default + Clone,
{
    bindings: Arena<Binding<'a, T>, DefaultArenaBehavior<Binding<'a, T>>>,
    tags: HashMap<TagName<'a>, Tag<'a, T>>,
}

impl<'a, T> Universe<'a, T>
where
    T: Default + Clone,
{
    /// Creates a new [`Universe`] using a [`UniverseBuilder`]. If you would rather not use a
    /// builder, a [`Universe`] can be initialized using default fields using the
    /// [`Default::default`] trait method
    ///
    /// [`Universe`]: ./struct.Universe.html
    /// [`UniverseBuilder`]: ./struct.UniverseBuilder.html
    /// [`Default::default`]: https://doc.rust-lang.org/nightly/std/default/trait.Default.html#tymethod.default
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(&mut UniverseBuilder) -> &mut UniverseBuilder,
    {
        let mut builder = UniverseBuilder::default();
        f(&mut builder).build()
    }

    /// Populates the [`Universe`] with a new value using a [`BindingBuilder`], returning a
    /// [`UniverseEntry`] referencing the given element. If a value with that name already exists,
    /// a [`UniverseError`] is returned.
    ///
    /// [`Universe`]: ./struct.Universe.html
    /// [`BindingBuilder`]: ./struct.ValueBuilder.html
    /// [`UniverseEntry`]: ./struct.UniverseEntry.html
    /// [`UniverseError`]: ./struct.UniverseError.html
    pub fn insert<F>(&mut self, f: F) -> Result<UniverseEntry<'a, T>, UniverseError>
    where
        F: for<'b> FnOnce(&'b mut BindingBuilder<'a, T>) -> &'b mut BindingBuilder<'a, T>,
    {
        let mut builder = BindingBuilder::default();
        f(&mut builder);

        let binding = self.bindings.alloc(builder.binding.clone());

        for t in &builder.tags {
            match self.tags.get_mut(t) {
                Some(t) => match t {
                    Tag::Primary(s) => {
                        let bindings = &self.bindings;

                        if s.iter()
                            .find(|b| match bindings.get(**b) {
                                Some(b) => b.name == builder.binding.name,
                                None => false,
                            })
                            .is_none()
                        {
                            s.push(binding);
                        } else {
                            return Err(UniverseError::BindingAlreadyExists);
                        }
                    }
                    Tag::Secondary(s) => s.push(binding),
                },
                None => match t {
                    TagName::Primary(_) => {
                        self.tags.insert((*t).clone(), Tag::Primary(vec![binding]));
                    }
                    TagName::Secondary(_) => {
                        self.tags
                            .insert((*t).clone(), Tag::Secondary(vec![binding]));
                    }
                },
            }
        }

        Ok(UniverseEntry {
            binding,
            tags: builder.tags,
        })
    }

    /// Retrives a reference to a [`Binding`] from the internal [`Arena`] using the provided id
    ///
    /// [`Binding`]: ./struct.Binding.html
    /// [`Arena`]: https://docs.rs/id-arena/2.2.1/id_arena/struct.Arena.html
    pub fn get(&self, id: <DefaultArenaBehavior<Binding<'a, T>> as ArenaBehavior>::Id) -> Option<&Binding<'a, T>> {
        self.bindings.get(id)
    }

    /// Retrieves a mutable reference to a [`Binding`] from the internal [`Arena`] using the
    /// provided id
    ///
    /// [`Binding`]: ./struct.Binding.html
    /// [`Arena`]: https://docs.rs/id-arena/2.2.1/id_arena/struct.Arena.html
    pub fn get_mut(&mut self, id: <DefaultArenaBehavior<Binding<'a, T>> as ArenaBehavior>::Id) -> Option<&mut Binding<'a, T>> {
        self.bindings.get_mut(id)
    }

    /// Performs an operation over the [`Universe`] using a [`UniverseOperationBuilder`] and
    /// returns the resulting [`Tag`]
    ///
    /// [`Universe`]: ./struct.Universe.html
    /// [`UniverseOperationBuilder`]: ./struct.UniverseOperationBuilder.html
    /// [`Tag`]: ./enum.Tag.html
    pub fn execute<F>(&mut self, f: F) -> Result<Tag<T>, UniverseError>
    where
        F: for<'b> FnOnce(
            &'b mut UniverseOperationBuilder<'a>,
        ) -> &'b mut UniverseOperationBuilder<'a>,
    {
        let mut builder = UniverseOperationBuilder::default();
        f(&mut builder);

        let tags = builder.tag_names.ok_or(UniverseError::NoTagsProvided)?;
        let sets = (
            self.tags
                .get(&tags.0)
                .ok_or(UniverseError::InvalidTagName)?
                .as_set()?,
            self.tags
                .get(&tags.1)
                .ok_or(UniverseError::InvalidTagName)?
                .as_set()?,
        );

        macro generate_length_and_operation_match_clause($sets:ident, $op:ident) {{
            let mut vec = Vec::new();
            OpBuilder::new($sets.0, $sets.1)
                .$op()
                .extend_collection(&mut vec);
            vec
        }}

        let (set, op) = match builder.op {
            Some(UniverseOperationOp::Union) => (
                generate_length_and_operation_match_clause!(sets, union),
                UniverseOperationOp::Union,
            ),
            Some(UniverseOperationOp::Intersection) => (
                generate_length_and_operation_match_clause!(sets, intersection),
                UniverseOperationOp::Intersection,
            ),
            Some(UniverseOperationOp::Difference) => (
                generate_length_and_operation_match_clause!(sets, difference),
                UniverseOperationOp::Difference,
            ),
            Some(UniverseOperationOp::SymmetricDifference) => (
                generate_length_and_operation_match_clause!(sets, symmetric_difference),
                UniverseOperationOp::SymmetricDifference,
            ),
            None => return Err(UniverseError::NoOperationProvided),
        };

        let group_composition = match tags {
            (TagName::Primary(_), TagName::Primary(_)) => TagGroupComposition::Primary,
            (TagName::Primary(_), TagName::Secondary(_)) => TagGroupComposition::PrimaryAndSecondary,
            (TagName::Secondary(_), TagName::Secondary(_)) => TagGroupComposition::Secondary,
            (TagName::Secondary(_), TagName::Primary(_)) => TagGroupComposition::SecondaryAndPrimary,
        };

        Ok(match (group_composition, op) {
            (
                TagGroupComposition::Primary
                | TagGroupComposition::PrimaryAndSecondary
                | TagGroupComposition::Secondary
                | TagGroupComposition::SecondaryAndPrimary,
                UniverseOperationOp::Union,
            ) => Tag::Secondary(set),

            (
                TagGroupComposition::Primary
                | TagGroupComposition::PrimaryAndSecondary
                | TagGroupComposition::SecondaryAndPrimary,
                UniverseOperationOp::Intersection,
            ) => Tag::Primary(set),
            (TagGroupComposition::Secondary, UniverseOperationOp::Intersection) => {
                Tag::Secondary(set)
            }

            (
                TagGroupComposition::Primary | TagGroupComposition::PrimaryAndSecondary,
                UniverseOperationOp::Difference,
            ) => Tag::Primary(set),
            (
                TagGroupComposition::Secondary | TagGroupComposition::SecondaryAndPrimary,
                UniverseOperationOp::Difference,
            ) => Tag::Secondary(set),

            (TagGroupComposition::Primary, UniverseOperationOp::SymmetricDifference) => {
                Tag::Primary(set)
            }
            (
                TagGroupComposition::PrimaryAndSecondary
                | TagGroupComposition::Secondary
                | TagGroupComposition::SecondaryAndPrimary,
                UniverseOperationOp::SymmetricDifference,
            ) => Tag::Secondary(set),
        })
    }
}
