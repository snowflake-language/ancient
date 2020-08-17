# parser

This directory has code for lexing, parsing, and ast.

Currently the parsing process is:

- lex str using logos
- parse lexed tokens into ast using lalrpop
- todo: ast may need to be converted into a better form ast
