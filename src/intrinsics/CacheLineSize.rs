// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Cache line size for compiled architecture.
#[cfg(target_arch = "x86_64")] pub const CacheLineSize: usize = 64;

/// Cache line size for compiled architecture.
#[cfg(target_arch = "x86")] pub const CacheLineSize: usize = 32;

/// Cache line size for compiled architecture.
#[cfg(target_arch = "aarch64")] pub const CacheLineSize: usize = 64;

/// Cache line size for compiled architecture.
#[cfg(all(target_pointer_width = "32", not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64"))))] pub const CacheLineSize: usize = 32;

/// Cache line size for compiled architecture.
#[cfg(all(target_pointer_width = "64", not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64"))))] pub const CacheLineSize: usize = 64;
