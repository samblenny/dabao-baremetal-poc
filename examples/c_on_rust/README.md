# Examples: C on Rust

The examples in this directory demonstrate how to do basic GPIO and serial
operations with C application code calling Rust IO drivers through an FFI
wrapper.

Key Tasks:
- Modify Bao1x Rust drivers to present a C FFI API
- Build binary where application logic happens in C using Rust IO drivers
- Document Rust code for setting up C FFI wrapper
- Document build procedure for C code linked with Rust code
