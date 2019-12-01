# snowflake

## a fast, minimal and strongly-typed programming language

```snowflake
** this is a comment
let main =>
	** printing is done through a macro, like rust
	*println "hello world"

	** underneath the hood, printing is done like this
	<stdout handle> <- "hello world" <- "\n"
```

