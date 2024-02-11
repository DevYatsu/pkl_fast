# Pkl_fast

## Overview
*pkl_fast* is a Rust library aiming to become an efficient tool for working with Apple's Pkl format. The library shall provide fast and reliable parsing and manipulation capabilities for Pkl files in Rust projects.

**Note**: This library is still in development. The lexer part is completed, and work on the parser has begun.

## Features
- Efficient lexer for tokenizing Pkl files (using the blazingly fast [logos](https://github.com/maciejhirsz/logos) crate)
- The parser is currently under development and is intended to return the Pkl file as statements. It will also provide warnings for non-recommended code practices and display errors as needed.
- Designed for speed and reliability.

## Contact
For questions, suggestions, or feedback, please reach out to me at <yatsu.dev@gmail.com>. Pull requests are appreciated, and issues are welcome too! Thank you for your interest in contributing to the project!
