# snowflake

a fast, minimal and strongly-typed lisp dialect where the core library is a
general purpose "dsl" that looks like an ml descendant

## experimental notice

for now, a lot of the syntax is entirely fluid and **can change at any moment**.
do not expect something that you made as an example of the language's features to
actually work once a compiler is made

## hello, world!

```snowflake
** this is a comment
let main =>
	** printing is done through a macro, like rust
	*println "hello world"

	** underneath the hood, printing is done like this
	<stdout handle> <- "hello world" <- "\n"
```

## potential implementation of [question()](https://github.com/superwhiskers/question)

```snowflake
** pretend that io stuff is imported
let question = prompt: &str, valid: [&str] => **[ the stuff beforehand is just a loose approximation ]**
	let =>
		input: String, ** this should automatically be initialized to the default value
		joined_valid = (valid.join ", "),
		reader = io:Reader:new,
	_ =>
		*println prompt
		if valid.length != 0 =>
			*print "(" joined_valid ")"
		*print ": "

	** TODO: finish
```

this translates (roughly, the lisp dialect's syntax isn't fully designed yet) to
the following

```
** pretend that io stuff is imported
(let question (Block:*new ((prompt :&str) (valid :[&str])) **[ the stuff beforehand is a loose approximation of what it may be ]**
	(let
		(input :String) ** this will be initialized to the default value
		(joined_valid :String (valid.join ", ")) ** because of transpilation, the type will be deambiguated
		(reader :io:Reader io:Reader:new))
	(let Block:*new
		(*println prompt)
		(if (not (valid.length.eq 0)) (Block:*new
			(*print "(" joined_valid ")")))
		(*print ": "))

	** TODO: finish
	))
```

## features

- low-level (relative to other languages) but with the syntax of a high level language
- mostly functional in terms of paradigm
- heavily abstractable (operator overloading, traits (potentially), declarative macros, procedural macros, [**type macros**](#type-macros))
- fast (to program in *and* during runtime)

## repositories

- [spec](https://github.com/snowflake-language/spec) - the (more formal) specification
