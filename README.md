# Pkl_fast

## Overview

_pkl_fast_ is a Rust library aiming to become an efficient tool for working with Apple's Pkl format. The library shall provide fast and reliable parsing and manipulation capabilities for Pkl files in Rust projects.

**Note**: This library is still in development. The lexer part is completed, and work on the parser has begun.

## Features

- Efficient lexer for tokenizing Pkl files (using the blazingly fast [logos](https://github.com/maciejhirsz/logos) crate)
- The parser is currently under development and is intended to return the Pkl file as statements. It will also provide warnings for non-recommended code practices and display errors as needed.
- Designed for speed and reliability.

## Todo List

- [x] Parsing basic values (e.g., int, float, boolean, string, datasize, duration)
- [x] Parsing objects declaration
- [x] Parsing `Map`, `Set`, `List`
- [ ] Parsing `Mapping`, `Listing`

- [x] Parsing simple type annotations (`Int`, `Float`, `UInt16`, `unknown`, `Any`)
- [x] Parsing more complex type annotations (e.g., `Listing<Type>`, `Mapping<Type, OtherType>`) without type checking
- [ ] Adding type checking

- [x] Parsing `import/import\*` statement (with optional `as`)
- [x] Parsing `amends` statement
- [x] Parsing `extends` statement
- [x] Parsing `module` statement

- [ ] Parsing variable declaration statement (partially done)
- [x] Parsing `class` statement
- [ ] Parsing `function` statement

- [x] Parsing `@ModuleInfo`/`@Deprecated` annotation
- [x] Parsing `typealias` annotation (partially done, need to support unions)
- [ ] Parsing If/Else/Let/For/When statement

- [x] Parsing Arithmetic expressions
- [x] Parsing Function call expressions
- [x] Parsing InterpolatedString expressions

- [ ] Generating a symbol table from the statements

## How to implement in Rust ?

Here is a list of pkl's features that should be implemented in rust, but which are not obvious to implement.

None for the moment, but there will be for sure...

## Contact

For questions, suggestions, or feedback, please reach out to me at <yatsu.dev@gmail.com>. Pull requests are appreciated, and issues are welcome too! Thank you for your interest in contributing to the project!
