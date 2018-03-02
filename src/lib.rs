// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of persistent-memory, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![allow(tyvar_behind_raw_pointer)]
#![deny(missing_docs)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(attr_literals)]
#![feature(box_into_raw_non_null)]
#![feature(cfg_target_feature)]
#![feature(collections_range)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(exact_size_is_empty)]
#![feature(fused)]
#![feature(i128_type)]
#![feature(inclusive_range)]
#![feature(integer_atomics)]
#![feature(link_llvm_intrinsics)]
#![feature(offset_to)]
#![feature(optin_builtin_traits)]
#![feature(pattern)]
#![feature(placement_new_protocol)]
#![feature(platform_intrinsics)]
#![feature(pointer_methods)]
#![feature(shared)]
#![feature(specialization)]
#![feature(static_nobundle)]
#![feature(stmt_expr_attributes)]
#![feature(str_internals)]
#![feature(target_feature)]
#![feature(thread_local)]
#![feature(trusted_len)]
#![feature(unicode)]
#![feature(unique)]
#![feature(untagged_unions)]


//! # persistent-memory
//!
//! This crate provides mid-level Rust wrappers for working with persistent memory.
//!
//! This crate makes extensive use of instructions of modern CPUs, particularly Skylake.
//!
//! To ensure these instructions are used, build with `cargo rustc -- -C target-feature=+sse2,+rdrnd,+clflushopt,+clwb`.
//!


extern crate alloc;
extern crate errno;
#[cfg(target_os = "windows")] extern crate kernel32;
extern crate libc;
pub extern crate parking_lot;
#[macro_use] extern crate quick_error;
#[cfg(not(all(target_feature = "rdrnd", any(target_arch = "x86", target_arch = "x86_64"))))] extern crate rand;
extern crate spin_locks;
extern crate std_unicode;
extern crate syscall_alt;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] #[macro_use] extern crate text_io;
#[cfg(target_os = "windows")] extern crate winapi;


use ::std::ptr::NonNull;


/// CTO Pool
pub mod cto_pool;

/// Path support for DAX (Direct Access) devices.
pub mod dax;

/// Hyper Thread support functions.
pub mod hyper_thread;

/// Essential Intrinsics.
pub mod intrinsics;

/// Specialized jemalloc support.
pub mod jemalloc;

/// Memory map (`mmap`) support functionality.
pub mod memory_map;

/// Persistent memory operations.
///
/// 1. Immediately after a `store`, write back the written value by issuing a pwb().
/// 2.a. Immediately before a `store-release` issue a `pfence()`.
/// 2.b. Immediately after a `store-release` write-back the written value by issuing a `pwb()`.
/// 3. Immediately after a `load-acquire` write-back the loaded value by issuing a `pwb()` followed by a `pfence()`.
/// 4a. Handle `CAS-acquire-release` as a combination of `store-release` and `load-acquire`:-
/// 	- immediately before the CAS, issue a `pfence()`
///  - immediately after the CAS,  write-back the loaded value by issuing a `pwb()` followed by a `pfence()`.
/// 4b. As for 4a, but also for `fetch_add` and `exchange` and probably all other read-modify-write instructions.
/// 5. Do nothing for `load`.
/// 6. Before taking any I/O action, issue a `psync()` to ensure all changes have reached persistent storage.
/// 7. Pedro Ramalhete & Andreia Correia argue that (4) does not require a `pfence()` before and a `pfence()` after on x86_64 because read-modify-write instructions (CAS, fetch_add, exchange, etc) ensure order for `clflushopt` and `clwb`.
pub mod persistent_memory_operations;


include!("Alignment.rs");
include!("ExtendedNonNull.rs");
include!("IsNotNull.rs");
include!("ToNonNull.rs");
