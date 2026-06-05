# seL4 Kernel - Rust Rewrite

This directory contains the Rust implementation of the seL4 microkernel.

## Overview

This is a work-in-progress effort to rewrite the seL4 kernel from C to Rust. The rewrite aims to improve:

- **Memory Safety**: Leverage Rust's ownership system to prevent entire classes of bugs
- **Concurrency Safety**: Use Rust's type system to prevent data races
- **Performance**: Maintain the high-performance characteristics of the original C implementation

## Building

To build the Rust kernel:

```bash
cd rust
cargo build --release
```

To run tests:

```bash
cargo test
```

## Project Structure

- `src/lib.rs` - Core kernel library
- `src/main.rs` - Kernel entry point
- `Cargo.toml` - Rust package configuration

## Contributing

This rewrite is in its early stages. The directory structure and module organization will evolve as the project progresses.

## License

The Rust implementation maintains the same license as the original seL4 kernel (BSD-2-Clause).
