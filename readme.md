<p align="center">
  <img align="center" src="https://github.com/Multimegamander/snowflake/blob/master/banner.jpg?raw=true" alt="banner">
</p>

<h2 align="center">a fast, minimal and strongly-typed lisp dialect where the core library is a
general purpose "dsl" that looks like an ml descendant 
</h2>

<p align="center">
	Made with :heart: and :coffee: by <a href="https://github.com/snowflake-language/snowflake/graphs/contributors">these</a> lovely people!
</p>

## 📖 Table of Contents
* [About the Project](#about-the-project)
  * [Built With](#built-with)
* [Usage](#usage)
  * [Experimental Notice](#experimental-notice)
  * [Hello, World!](#hello-world)
  * [Question Implementation](#potential-implementation-of-question)
  * [Features](#features)
* [Roadmap](#roadmap)
* [Contributing](#contributing)
* [License](#license)
* [Contact](#contact)
* [Repositories](#repositories)

## 🤔 About the Project

### 🔨 Built With

## 💡 Usage

## 🧪 Experimental Notice

for now, a lot of the syntax is entirely fluid and **can change at any moment**.
do not expect something that you made as an example of the language's features to
actually work once a compiler is made

### 👋 Hello, World!
 
```snowflake
** this is a comment
let main =>
	** printing is done through a macro, like rust
	*println "hello world"

	** underneath the hood, printing is done like this
	<stdout handle> <- "hello world" <- "\n"
```

### ❓ Potential Implementation of [Question()](https://github.com/superwhiskers/question)

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

### 🌟 Features

- low-level (relative to other languages) but with the syntax of a high level language
- mostly functional in terms of paradigm
- heavily abstractable (operator overloading, traits (potentially), declarative macros, procedural macros, [**type macros**](#type-macros))
- fast (to program in *and* during runtime)

## 🚧 Roadmap

## 🤷 Contributing

We are always open to contributions! If you want to help, be sure to look at the [Contributing.md](https://github.com/snowflake-language/snowflake/blob/master/Contributing.md)

## 🖊️ License

## 💬 Contact

## 🎁 Repositories

- [spec](https://github.com/snowflake-language/spec) - the (more formal) specification
