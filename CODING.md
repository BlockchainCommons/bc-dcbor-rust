# dCBOR (Deterministic CBOR) Guidelines

## Project Overview

This crate is the Rust reference implementation of dCBOR (deterministic CBOR).

## Development Environment

### Build/Test Commands

```bash
# Build a specific crate
cargo build

# Run tests
cargo test
cargo test --test <test_name>
cargo test --doc

# Check code quality
cargo clippy -- -D warnings

# Build documentation
cargo doc --no-deps --target-dir cargo-docs
```

### Development Guidelines

- **Production quality** - Write code as you would for a real-world implementation
- **Proper error handling** - Use `Result<T>` with `anyhow::Context` for all functions that can fail
- **Clean code** - Fix all compiler errors and Clippy lints

### Testing

- Don't mark tasks as complete until all tests pass

## Core Reference: dCBOR Types and Concepts

### Key Data Types

| Type          | Description                                               |
| ------------- | --------------------------------------------------------- |
| `CBOR`        | A reference-counted `CBORCase` object
| `CBORCase`    | An enum representing a CBOR case with various data types |

### Reference Materials

These documents are in the codebase for reference. Always refer to the latest version of the dCBOR specification for accurate information.

🚨 **NOTE**: Always make sure you have the dCBOR Internet Draft, as well the CDE I-D in your context, as this crate is a reference implementation of that specification.

| Title | Description | URL |
|-------|-------------|-----|
| dCBOR Internet Draft | IETF draft specification for Deterministic CBOR | [docs/draft-mcnally-deterministic-cbor.txt](docs/draft-mcnally-deterministic-cbor.txt) |
| CDE | CBOR Common Deterministic Encoding (CDE) | [docs/draft-ietf-cbor-cde.txt](docs/draft-ietf-cbor-cde.txt) |
| bcr-2023-008-dcbor-date | Blockchain Commons Research document serializing CBOR dates in dCBOR | [docs/bcr-2023-008-dcbor-date.md](docs/bcr-2023-008-dcbor-date.md) |

## Current Status and Roadmap

### 🟢 In Progress: Comprehensive Crate Documentation

- **Goal**: Enhance the documentation for the `dcbor` crate to improve usability and understanding. Document all public API elements, including structs, enums, and functions.

#### Documentation Quality Criteria

- **Comprehensive**: All public API elements have documentation
- **Contextual**: Documentation explains both "what" and "why"
- **Practical**: Examples demonstrate real-world usage
- **Consistent**: Uniform style and detail level across the codebase
- **Accessible**: Explanations suitable for developers not familiar with Rust, and Rust engineers not familiar with CBOR
- **Searchable**: Proper cross-references and keyword usage
- **Validated**: Examples compile and work correctly

#### Documentation Testing Guidelines

- **Doc Example Best Practices:**
  - Use `dcbor::prelude::*` in all examples
  - Handle errors properly in examples that return `Result`
  - Use `no_run` for examples that can't be directly compiled/run in doc tests
  - Check constructors for type initialization in examples - some types may lack `Default` implementation
  - When showing how to work with boolean values, use Rust's native `true`, `false` values or the convenience methods `CBOR::r#true()`, `CBOR::r#false()`, and `CBOR::null()`
  - Don't import or use implementation details like `Simple` directly in examples - focus on the public API
  - For internal/implementation types that users shouldn't directly interact with, clearly mark them as such in the documentation
  - Show typical usage patterns for each type, not all possible ways to use it

#### Required Quality Checks

🚨 **CRITICAL**: Always perform these quality checks with EVERY documentation task:

1. **Fix all doc tests**:
   ```bash
   cargo test --doc
   ```
   Ensure all doc tests pass, and fix any failures immediately.

2. **Fix all Clippy lints**:
   ```bash
   cargo clippy -- -D warnings
   ```
   Address any Clippy warnings introduced by documentation changes.

### Public API Items Needing Documentation

This section inventories all public API items that need documentation, ordered from simplest with least dependencies to most complex.

#### Core Data Types and Errors

1. **✅ `CBORError`** (`error.rs`) - Error types for CBOR operations
2. **✅ `Simple`** (`simple.rs`) - CBOR simple values (true, false, null, float)
3. **✅ `TagValue`** (`tag.rs`) - Type alias for CBOR tag values
4. **✅ `Tag`** (`tag.rs`) - CBOR tag representation
5. **✅ `ByteString`** (`byte_string.rs`) - Byte string representation

#### Traits

1. **✅ `CBOREncodable`** (`cbor_codable.rs`) - Trait for types that can be encoded to CBOR
2. **✅ `CBORDecodable`** (`cbor_codable.rs`) - Trait for types that can be decoded from CBOR
3. **✅ `CBORCodable`** (`cbor_codable.rs`) - Combined trait for types that can be both encoded and decoded
4. **✅ `CBORTagged`** (`cbor_tagged.rs`) - Trait for types with associated CBOR tags
5. **✅ `CBORTaggedEncodable`** (`cbor_tagged_encodable.rs`) - Trait for encoding tagged types to CBOR
6. **✅ `CBORTaggedDecodable`** (`cbor_tagged_decodable.rs`) - Trait for decoding tagged types from CBOR
7. **✅ `CBORTaggedCodable`** (`cbor_tagged_codable.rs`) - Combined trait for tagged types that can be both encoded and decoded

#### Core CBOR Types

1. **✅ `CBOR`** (`cbor.rs`) - Main symbolic representation of CBOR data
2. **✅ `CBORCase`** (`cbor.rs`) - Enum representing the different types of CBOR data
3. **✅ `Date`** (`date.rs`) - Date representation in CBOR

#### Collections and Utilities

1. **✅ `MapIter`** (`map.rs`) - Iterator over CBOR map entries
2. **✅ `Map`** (`map.rs`) - CBOR map implementation
3. **✅ `CBORSummarizer`** (`tags_store.rs`) - Type for summarizing CBOR values
4. **✅ `TagsStoreTrait`** (`tags_store.rs`) - Trait for mapping between tags and their names
5. **✅ `TagsStore`** (`tags_store.rs`) - Dictionary of mappings between tags and their names

#### Functions

1. **✅ `with_tags`** - Macro for working with the global tags registry in a read-only manner
2. **✅ `with_tags_mut`** - Macro for working with the global tags registry in a mutable manner
3. **✅ `tags_for_values`** - Function to convert tag values to Tag objects using the global registry

### API Design Insights

The following insights about the API design of this crate have been learned during documentation:

1. **Public API vs Implementation Details**: Some types like `Simple` are implementation details that users generally shouldn't interact with directly. The public API favors working with Rust's native types (`bool`, `f64`, etc.) which are converted to/from CBOR.

2. **Convenience Methods**: The library provides convenience methods for common CBOR values:
   - `CBOR::r#true()`, `CBOR::r#false()`, and `CBOR::null()`
   - These should be highlighted in documentation as the preferred way to create these values

3. **Deterministic Encoding Validation**: The library strictly validates deterministic encoding according to the dCBOR specification - this should be emphasized as a key feature.

4. **Type Conversions**: The library has rich conversion support between Rust and CBOR types, which should be documented with clear examples.

5. **Error Handling**: Error handling is comprehensive and specific, with detailed errors for each potential violation of the dCBOR specification.

6. **Idiomatic Rust Conversions**: The library makes extensive use of Rust's type conversion traits:
   - `From<T>` for types that can be converted to CBOR without error
   - `TryFrom<CBOR>` for types that can be decoded from CBOR with possible errors
   - These enable the ergonomic `.into()` and `.try_into()` patterns

7. **Multiple Tags for Backward Compatibility**: The `CBORTagged` trait allows specifying multiple tags:
   - The first tag is used for encoding (preferred/canonical tag)
   - All tags are accepted for decoding (enables backward compatibility)
   - This allows evolving from tags in the IANA "Specification Required" range (24-32767) to the "First Come First Served" range (32768+)

8. **Reference Counting**: The `CBOR` type uses reference counting (`Rc` or `Arc`, depending on the feature flags) to efficiently share data structures in memory.

9. **Thread Safety**: The library provides a `multithreaded` feature flag to use `Arc` instead of `Rc` for thread-safe reference counting.

10. **Date Handling**: Date values are stored as seconds since the Unix epoch, with proper support for fractional seconds and times before the epoch.

11. **Map Implementation**: The `Map` type provides a deterministic mapping implementation that:
    - Automatically sorts keys by their CBOR binary representation
    - Prevents duplicate keys
    - Supports type-safe value extraction with `get<K, V>()` and `extract<K, V>()`
    - Converts seamlessly from Rust's standard `HashMap` and `BTreeMap`

12. **Tag Registration System**: The library provides a comprehensive tag registration system:
    - `TagsStore` provides the core registry mapping between tag numbers and human-readable names
    - Global tag registry accessible via `with_tags!` and `with_tags_mut!` macros
    - Thread-safe access to the registry via mutex protection
    - Customizable tag summarizers for formatting tagged values in diagnostic output
    - Prevention of duplicate registrations with conflicting names
    - Graceful handling of unregistered tags with fallback mechanisms

### 🔵 FUTURE ENHANCEMENTS

None planned.
