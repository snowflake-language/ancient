# snowflake

a fast, minimal, and strongly-typed programming language with no hierarchy.

## experimental notice

for now, a lot of the syntax is entirely fluid and **can change at any moment**.
do not expect something that you made as an example of the language's features to
actually work once a compiler is finished.

## tagging
tagging is the core idea of snowflake. it's what allows snowflake to be non-hierarchical yet organized.

#### What is a tag?

A tag is, in essence, a mathematical set with a few changes. The most important of these changes is that there are two kinds: primary tags and secondary tags.

A primary tag is equivalent to a mathematical sets in all regards, with one exception: the name of the binding must be unique. A secondary tag is the same but without a uniqueness restriction.

There is one more change, and that is that due to how tags work (they're collections of bindings from a name to a value), the uniqueness restriction is only enforced on the name of the binding, and not at all on the value.

#### I don't understand this. Can you explain it a little simpler?

If you've ever used a venn diagram, they can be visualized in the same way. Tag intersections are where the components overlap, unions are any two components combined, symmetric difference (xor) is everything that doesn't overlap, and so on...

## examples
note: none of these work yet, sorry! this is just a demonstration of how things will likely end up once we get the interpreter implemented

#### hello, world!

```snowflake
** this is a comment
let main =>
	** printing is done through a macro, like rust
	*println "hello world"
```

#### potential implementation of [question()](https://github.com/superwhiskers/question)

```snowflake
** pretend that io stuff is imported
question :: &str [&str] -> Unit
question prompt, valid: [&str] => **[ the stuff beforehand is just a loose approximation ]**
	let =>
		input = "",
		joined_valid = (join ", " valid),
		reader => io:Reader,
	_ =>
		*println prompt

        ** no if (for now); that'll be implemented with a macro later
		match (len valid) != 0 =>
            True =>
			    *print "(" joined_valid "):"
            _ =>
                Unit
		*print ": "

        read_line input

        match (len valid) != 0 =>
            True =>
                return input
            _ =>
                Unit
        
        *foreach ele valid =>
            match ele == input =>
                True =>
                    return input
                _ =>
                    Unit
                    
        *print input " is not a valid answer!"
        question prompt valid
```

## features

- low-level (relative to other languages) but with the syntax of a high level language
- mostly functional in terms of paradigm
- heavily abstractable (operator overloading, traits (potentially), declarative macros, procedural macros, [**type macros**](#type-macros))
- fast (to program in *and* during runtime)
