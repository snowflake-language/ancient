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

use std::collections::HashMap;
use id_arena::{Arena, DefaultArenaBehavior};
use sdset::SetBuf;

/// A relation between a string and its corresponding value. The string is considered to be the
/// uniqueness specifier, the value has no play in equality
#[derive(Debug, Default)]
pub struct Binding<T> {
    pub name: String,
    pub value: T,
}

impl<T> PartialEq for Binding<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T> Eq for Binding<T> {}

/// A tag. Primary tags are equivalent to mathematical sets, secondary tags are the same but
/// without the uniqueness restriction. Within the contained [`Binding`], only the [`name`] must be
/// unique
///
/// [`Binding`]: ./struct.Binding.html
/// [`name`]: ./struct.Binding.html#structfield.name
#[derive(Debug)]
pub enum Tag<T> {
    Primary(SetBuf<Binding<T>>),
    Secondary(SetBuf<Binding<T>>),
}

/// A builder for the [`Universe`] type, designed to make constructing its components easier
#[derive(Debug, Default)]
pub struct UniverseBuilder<T> {
    pub(crate) universe: Universe<T>,
}

impl<T> UniverseBuilder<T> {
    /// "Builds" the builder type, returning the internal [`Universe`]
    ///
    /// [`Universe`]: ./struct.Universe.html
    fn build(self) -> Universe<T> {
        self.universe
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Universe<T> {
    bindings: Arena<Binding<T>, DefaultArenaBehavior<Binding<T>>>,
    tags: HashMap<String, SetBuf<Binding<T>>>, 
}

impl<T> Universe<T> {
}
