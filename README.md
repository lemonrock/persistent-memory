# persistent-memory

[persistent-memory] is a rust crate.

This crate provides mid-level Rust wrappers for working with persistent memory.

This crate makes extensive use of instructions of modern CPUs, particularly Skylake.

To ensure these instructions are used, build with `cargo rustc -- -C target-feature=+rdrnd,+sse2,+clflushopt,+clwb`.


## Licensing

The license for this project is MIT.

[persistent-memory]: https://github.com/lemonrock/persistent-memory "persistent-memory GitHub page"
