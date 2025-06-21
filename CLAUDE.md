# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Development Commands

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run a specific test
cargo test test_name

# Build with optimizations
cargo build --release

# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Check code without building
cargo check

# Search -- grep replacement
rg

# Find files -- find replacelemt
fd

# Xargs
xargs

# Sed
sed

# Awk
awk

# Find & replacele
sd

# Cat
cat
```

## Writing Instructions

These instructions apply to any communcation (e.g. feedback you print to the user) as well as any documentation you write.

- Be concise

- Use simple sentences. But feel free to use technical jargon.

- Do NOT overexplain basic concepts. Assume the user is technically proficient.

- AVOID flattering, corporate-ish or marketing language. Maintain a neutral viewpoint.

- AVOID vague and/or generic claims which may seem correct but are not substantiated by the the context.

- AVOID weasel words.

## Code

- Write idiomatic and canonical Rust code. I.e. avoid patterns common in imperative languages like C/C++/JS/TS that can be expressed more elegantly, concise and with more leeway for the compiler to optimize, in Rust.

- PREFER functional style over imperative style. I.e. use for_each or map instead of for loops, use collect instead of pre-allocating a Vec and using push.

- USE rayon to parallelize whenever larger amounts of data are being processed.

- AVOID unnecessary allocations, conversions, copies

- AVOID using `unsafe` code unless absolutely necessary.

- AVOID return statements; structure functions with if ... if else ... else blocks instead.

- Prefer using the stack, use SmallVec whenever it makes sense.

- NAMING follows the rules laid out in this document: https://raw.githubusercontent.com/rust-lang/api-guidelines/refs/heads/master/src/naming.md

## Documentation

- All code comments MUST end with a period.

- All doc comments should also end with a period unless they're headlines. This includes list items.

- ENSURE an en-dash is expressed as two dashes like so: --. En-dashes are not used for connecting words, e.g. "compile-time".

- All references to types, keywords, symbols etc. MUST be enclosed in backticks: `struct` `Foo`.

- For each part of the docs, every first reference to a type, keyword, symbol etc. that is NOT the item itself that is being described MUST be linked to the relevant section in the docs like so: [`Foo`].

### Performance Requirements

- Use `rayon` for parallelism in bulk operations
- Prefer `par_iter()`/`par_iter_mut()` for operations with a lot of data (arrays, vectors, etc.)
- Avoid unnecessary allocations, conversions, copies

### Design Patterns

- Use Rust's type system as much as possibl to make code more idiomatic/generic
- Prefer enum dispatch over `dyn` traits
- Use generics and traits for type flexibility
- Employ macros to reduce code duplication
- Functional notation for bulk operations

## Traits

- All public-facing types must implement `Debug`, `Clone`, `PartialEq`, and `Copy` (the latter if directly derivable).
- Types with integer-based representations (e.g., `Color<SrgbU8>`) also implement `Eq` and `Hash`.
- Types with floating-point representations (e.g., `Color<LinearSrgb>`) only implement `PartialEq` due to floating-point precision considerations.
