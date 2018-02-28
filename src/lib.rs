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
#[cfg(target_os = "windows")] extern crate winapi;


use ::std::ptr::NonNull;


/// CTO Pool
pub mod cto_pool;

/// Hyper Thread support functions.
pub mod hyper_thread;

/// Essential Intrinsics.
pub mod intrinsics;


include!("ExtendedNonNull.rs");
include!("IsNotNull.rs");
include!("ToNonNull.rs");
