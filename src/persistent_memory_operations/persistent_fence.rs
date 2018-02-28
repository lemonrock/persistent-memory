// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// This is an implementation of the function `pfence` in the paper [Brief Announcement: Preserving Happens-before in Persistent Memory](https://www.cs.rochester.edu/u/jhi1/papers/2016-spaa-transform) which provides a generic scheme for working with persistent memory and atomic operations.
///
/// Enforces an ordering between previous and subsequent persistent-write-backs (`persistent_write_back()`s) in the current thread.
///
/// Acts as a memory fence.
///
/// * Use this immediately after `persistent_write_back()` when `persistent_write_back()` was for:-
///     * an *Acquire* `load` (on Intel, this is a `mfence()` followed by a `ld()`)
///     * A locked read-modify-write instructions, such as Compare-and-Swap, Fetch-Add, Exchange, which has *Acquire-Release* (or presumably stronger) memory ordering\*
/// * Use this immediately before:-
/// 	* A *Release* `store`
///     * A locked read-modify-write instructions, such as Compare-and-Swap, Fetch-Add, Exchange, which has *Acquire-Release* (or presumably stronger) memory ordering\*
///
/// Does not seem to be needed if `persistent_write_back()` is implemented using the `clflush()` intrinsic, which is the least efficient, oldest and worst choice for implementing `persistent_write_back()`.
///
/// \* Pedro Ramalhete & Andreia Correia argue that the preceding and following `persistent_fence()` are not needed on x86_64 because locked read-modify-write instructions ensure order for `clflushopt()` and `clwb()`.
#[inline(always)]
pub fn persistent_fence()
{
	// Intel have withdrawn the `pcommit` instruction.
	// Originally this code would have been `sfence(); pcommit(); sfence()` for transient memory buffers; see table 1 in the paper [Brief Announcement: Preserving Happens-before in Persistent Memory](https://www.cs.rochester.edu/u/jhi1/papers/2016-spaa-transform).
	#[cfg(all(target_feature = "sse2", any(target_arch = "x86_64", target_arch = "x86")))] sfence();
	
	#[cfg(target_arch = "aarch64")] dmb_ish();
}
