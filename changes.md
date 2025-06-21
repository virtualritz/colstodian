1. Corrected EOTF/OETF usage: Changed `srgb_eotf` to `srgb_oetf` in `dst_transform_raw` methods for `SrgbU8`, `SrgbF32`, `SrgbaU8`, `SrgbaF32`, and `SrgbaPremultipliedU8` in `/home/ritz/code/crates/colstodian/src/details/encodings.rs`.
2. Fixed rounding: Added `.round()` to `f32_to_u8` function.
3. Updated test expectations: Adjusted expected values in doctests to match the corrected rounding behavior (`127`→`128`, `206`→`207`).
4. Moved all tests to `tests` folder.
5. Added approx crate: Added `approx = "0.5"` as both a dev-dependency and optional feature dependency.
6. Implemented `approx` traits: Created implementations of `AbsDiffEq` and `RelativeEq` for:

   - `Color<E>`
   - `Rgb<f32>`, `Rgba<f32>`, `Rgb<u8>`, `Rgba<u8>`
   - `Lab<f32>`

7. Replaced all `assert_eq_eps!` calls: Updated all test files to use `assert_relative_eq!` with the original epsilon values (0.001, 0.0001, etc.)
8. Updated imports: Changed from use `colstodian::equals_eps::*` to use `approx::assert_relative_eq`
9. Updated doctests: Fixed all documentation examples to use the new approx macros
10. Removed custom code: Eliminated the custom `equals_eps` module and `assert_eq_eps!` macro entirely.
11. Added feature gate: Made the `approx` implementations available via the `approx` feature flag.
