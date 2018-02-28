// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// This is an implementation of the function `pwb addr` in the paper [Brief Announcement: Preserving Happens-before in Persistent Memory](https://www.cs.rochester.edu/u/jhi1/papers/2016-spaa-transform) which provides a generic scheme for working with persistent memory and atomic operations.
///
/// Initiates the write-back of the cache-line associated with address to persistent memory; on a 64-bit x86_64 chip, this is 64 bytes.
///
/// Non-blocking.
///
/// * Use this immediately after a *Relaxed* `store`.
/// * Use this immediately after a *Release* `store`.
/// * Use this immediately after an *Acquire* `load` (on Intel, this is a `mfence()` followed by a `ld()` or a locked read-modify-write instruction, which Linus argues is faster) (followed by a `persistent_fence()`).
/// * Use this immediately after all locked read-modify-write instructions, such as Compare-and-Swap, Fetch-Add, Exchange, etc (followed by a `persistent_fence()`\*) which have *Acquire-Release* (or presumably stronger) memory ordering; precede the locked read-modify-write instructions with a `persistent_fence()`\*.
///
/// \* Pedro Ramalhete & Andreia Correia argue that the preceding and following `persistent_fence()` are not needed on x86_64 because locked read-modify-write instructions ensure order for `clflushopt()` and `clwb()`.
#[allow(unused_variables)]
#[inline(always)]
pub fn persistent_write_back(address: *mut u8)
{
	let address_rounded_down_to_start_of_cache_line = round_address_down_to_start_of_cache_line(address);
	
	#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
	{
		if cfg!(target_feature = "clwb")
		{
			#[cfg(target_feature = "clwb")] clwb(address_rounded_down_to_start_of_cache_line)
		}
		else if cfg!(target_feature = "clflushopt")
		{
			#[cfg(target_feature = "clflushopt")] clflushopt(address_rounded_down_to_start_of_cache_line)
		}
		else if cfg!(target_feature = "sse2")
		{
			// Be aware that this ALSO does persistent_fence(), and so isn't equivalent to `clwb` or `clflushopt`.
			// However, we do not optimize for this case. All chips from Skylake onwards support `clflushopt`, and persistent memory NVDIMMs do not effectively exist for pre-Skylake.
			#[cfg(target_feature = "sse2")] clflush(address_rounded_down_to_start_of_cache_line)
		}
	}
	
	#[cfg(target_arch = "aarch64")]
	{
		dc_cvac(address_rounded_down_to_start_of_cache_line)
	}
}
