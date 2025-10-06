# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## The Golden Rule

When unsure about implementation details, ALWAYS ask the developer.

## Build and Development Commands

```bash
# Build the project
cargo build

# Build with specific features
cargo build --features "std,serde,bytemuck"
cargo build --no-default-features --features libm

# Run tests
cargo test

# Run tests with approx feature (needed for floating-point comparisons)
cargo test --features approx

# Run a specific test
cargo test test_name

# Run benchmarks
cargo bench

# Run a specific benchmark
cargo bench benchmark_name

# Build with optimizations
cargo build --release

# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Run clippy and fix issues automatically
cargo clippy --fix --allow-dirty

# Check code without building
cargo check

# Generate documentation
cargo doc --open

# Run example
cargo run --example basic_usage
cargo run --example color_math
cargo run --example game_graphics_workflow

# CRITICAL: Run tests and ensure clean build before committing
cargo test --all-features  # Must pass WITHOUT ANY WARNINGS
cargo build --all-targets --all-features  # Check examples/benches are warning-free
cargo fmt  # Format code
cargo clippy --all-targets --all-features -- -W warnings  # Fix any issues
cargo test --all-features  # Final verification
```

## Writing Instructions

These instructions apply to any communcation (e.g. feedback you print to the user) as well as any documentation you write.

- Be concise

- Use simple sentences. But feel free to use technical jargon.

- Do NOT overexplain basic concepts. Assume the user is technically proficient.

- AVOID flattering, corporate-ish or marketing language. Maintain a neutral viewpoint.

- AVOID vague and/or generic claims which may seem correct but are not substantiated by the the context.

- AVOID weasel words.

- All list items in documentation MUST be complete sentences that end with a period.

## High-Level Architecture

`colstodian` is a color management library focused on practical color handling for games and graphics. The core architecture revolves around:

1. **`Color<E>`** - The unified color type that is generic over a `ColorEncoding`.
2. **`ColorEncoding` trait** - Defines how color data is stored and interpreted (e.g., `SrgbU8`, `LinearSrgb`, `Oklab`).
3. **`ColorRepr` trait** - The underlying data representation (e.g., `Rgb<u8>`, `Rgba<f32>`, `Lab<f32>`).
4. **`LinearColorSpace` trait** - Defines the actual color space (primaries and white point).
5. **Conversion system** - Automatic conversions between encodings via `ConvertFrom` trait and `.convert::<E>()` method.

### Key Modules

- `details::encodings` - All color encoding implementations (sRGB, Linear sRGB, Oklab, etc.).
- `details::linear_spaces` - Linear color space definitions (Srgb, Bt2020, AcesCg, etc.).
- `details::component_structs` - Component types like `Rgb`, `Rgba`, `Lab`.
- `details::traits` - Core traits that power the type system.
- `details::color` - The main `Color<E>` type implementation.

### Working vs Display Encodings

- **Working encodings** (implement `WorkingEncoding`) - Used for color math operations (e.g., `LinearSrgb`, `Oklab`).
- **Display encodings** - Used for storage/display (e.g., `SrgbU8`, `SrgbF32`).
- Colors must be in a working encoding to support arithmetic operations.

### Feature Flags

- `std` (default) - Standard library support.
- `libm` - Required for `no_std` builds.
- `serde` - Serialization support.
- `bytemuck` - Zero-copy conversions.
- `approx` - Approximate equality testing (mainly for tests).

## Code Style and Patterns

### Guidelines

- **Test Naming Convention**: Test functions should NOT be prefixed with `test_`. The `#[test]` attribute already indicates it's a test.

- **CRITICAL: ALWAYS run `cargo test --all-features` and ensure code compiles and tests pass WITHOUT ANY WARNINGS before committing!** The code must be completely warning-free across all tests, examples, benches, and the library itself.

- **CRITICAL: Address ALL warnings before EVERY commit!** This includes:
  - Unused imports, variables, and functions.
  - Dead code warnings.
  - Deprecated API usage.
  - Type inference ambiguities.
  - Any clippy warnings or suggestions.
  - **NEVER prefix unused variables with underscore** -- remove them entirely.
  - **DO NOT keep dead code** -- if code is unused, delete it completely.

- **DO NOT change any public-facing API** without presenting a change proposal to the user first, including a rationale and getting permission.

## Code

- Write idiomatic and canonical Rust code. I.e. avoid patterns common in imperative languages like C/C++/JS/TS that can be expressed more elegantly, concise and with more leeway for the compiler to optimize, in Rust.

- PREFER functional style over imperative style. I.e. use for_each or map instead of for loops, use collect instead of pre-allocating a Vec and using push.

- USE rayon to parallelize whenever larger amounts of data are being processed.

- AVOID unnecessary allocations, conversions, copies.

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

- NEVER use fully qualified paths in doc links. Use [`Foo`] instead of [`foo::bar::Foo`].

- NEVER use crate paths in doc link references. Wrong: `[`crate::Color`]`. Right: `[`Color`]`.

### Performance Requirements

- Use `rayon` for parallelism in bulk operations.
- Prefer `par_iter()`/`par_iter_mut()` for operations with a lot of data (arrays, vectors, etc.).
- Avoid unnecessary allocations, conversions, copies.

### Design Patterns

- Use Rust's type system as much as possible to make code more idiomatic/generic.
- Prefer enum dispatch over `dyn` traits.
- Use generics and traits for type flexibility.
- Employ macros to reduce code duplication.
- Functional notation for bulk operations.

## Traits

- All public-facing types must implement `Debug`, `Clone`, `PartialEq`, and `Copy` (the latter if directly derivable).
- Types with integer-based representations (e.g., `Color<SrgbU8>`) also implement `Eq` and `Hash`.
- Types with floating-point representations (e.g., `Color<LinearSrgb>`) only implement `PartialEq` due to floating-point precision considerations.
