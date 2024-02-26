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
- [ ] Parsing Objects, Class
- [ ] Parsing Map, Mapping, Set, Listing

- [x] Parsing simple type annotations (`Int`, `Float`, `UInt16`, `unknown`, `Any`)
- [x] Parsing more complex type annotations (e.g., `Listing<Type>`, `Mapping<Type, OtherType>`)

- [x] Parsing import/import\* statement (with optional `as`)
- [x] Parsing amends statement
- [x] Parsing extends statement
- [x] Parsing module statement

- [ ] Parsing variable declaration statement (partially done)
- [ ] Parsing variable class statement
- [ ] Parsing functions statement

- [ ] Parsing @ModuleInfo/@Deprecated annotation
- [ ] Parsing typealias annotation
- [ ] Parsing If/Else/Let/For/When statement

- [ ] Parsing Arithmetic expressions
- [ ] Parsing Function call expressions
- [ ] Parsing InterpolatedString expressions

- [ ] Generating a symbol table from the statements

## Contact

For questions, suggestions, or feedback, please reach out to me at <yatsu.dev@gmail.com>. Pull requests are appreciated, and issues are welcome too! Thank you for your interest in contributing to the project!
