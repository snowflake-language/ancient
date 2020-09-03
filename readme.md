# snowflake

a fast, low-level, and expressive programming language designed for minimal usage of hierarchy

## a brief history

snowflake, the language, originally started as a collection of ideas to improve programming that
somewhat resembled a mashup of go and rust (more heavily influenced by go at the time, but with the
lower-level-ness of rust and some additional features superwhiskers, the author, thought would be
nice to have in go).


![snowflake in 2018](https://256.sh/i/5drn734b.png)


further on in conceptualization, it gradually shifted to be more and more rust-like, until the
point where it became rust but with a few tweaks such as the removal of ownership to allow for
fully manual memory management by default, and other extensions (that in hindsight wouldn't make
sense to create what was essentially a fork to implement).


eventually, it shifted to become more like lisp (retaining strong typing and manually managed
memory) and had some things, such as most types and other unnecessary (in superwhiskers' opinion)
language items/features stripped out to make a really portable language. in addition, macros were
buffed to add in an _all-powerful_ file-wide macro kind that could be used to implement alternative
syntaxes on top of a lispy syntax to allow people to use the language the way they wanted


![snowflake but lispy](https://256.sh/i/v2936j95.png)

![snowflake but lispy (no methods)](https://256.sh/i/6tpe2hzh.png)


later on, methods were removed (as seen above) and tagging was created/discovered/applied to the
language. this discovery heavily influenced the language later on, as even though superwhiskers was
hesitant to apply it everywhere at first, it gradually made its way in, creating the language you
see today


![snowflake but ml](https://256.sh/i/6mjmmz7j.png)


more recently, the lispy syntax was outright removed to simplify the parser and make it easier to
implement, but alternative syntaxes are not completely gone, as they are now intended to be done
using plugins. however, technically speaking, the current ml-like syntax you see now has existed
since the lispy one, as it was intended to be the "default syntax", used in most cases as it would
be drastically easier to work with (typed lisps are a pain).

## the language itself

snowflake is a language designed from the start to be low level. ideally, it should take very
little to port snowflake to a new platform than c (due to very little types existing in the
language itself and because it assumes very little about the underlying platform). aside from that,
other goals/features include (but are not limited to):

- non-hierarchical programming (in both module system and type system)
- speed (due to the expressiveness of macros and such + optimizations + low-level-ness, one can get
  more performance out of equivalent code in other languages without needing to rely on the
  implementation)
- a primarily functional programming style (without sacrificing speed)
- clean, easy to read syntax that should be familiar to users of other functional languages

## an explaination of tagging

tagging is essentially applied set theory; it is based entirely upon single-layer collections of
objects that can have operations applied to them in order to construct new tags. these operations
can be listed as such:

- intersection, which returns a tag containing all of the common items between the operands
- union, which returns a tag containing all of the items in both sets regardless of presence
- difference, which returns a tag containing the items in the first set minus the ones that exist
  in the second
- symmetric difference/xor, which returns all of the items not in both sets

aside from that, snowflake tweaks the set theory model to create two kinds of tags: primary and
secondary tags. primary tags are just mathematical sets, and secondary tags are mathematical sets
without a uniqueness restriction. there is also one more change, the sets exclusively contain
name-value bindings (like a map/key-value store/whatever) and primary tags only need to be unique
on the name

if you don't understand this, you can visualize it like how set theory is taught really early on in
grade school: as a [venn diagram](https://en.wikipedia.org/wiki/Venn_diagram), where intersections
are the overlapping parts of components, unions are two components combined, etc...

## examples

### hello world

```snowflake
def main =>
    *println "hello, world"
```

(there isn't much else that isn't likely to change)

## links

- [the repl.it team](https://repl.it/@snowflakelang)
- [the discord guild](https://discord.gg/rBbfDEr)
- [the telegram group](https://t.me/joinchat/GwKOHRzeLzT2Jktw_4SeVg)
- [the github organization](https://github.com/snowflake-language)
