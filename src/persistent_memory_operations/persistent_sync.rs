// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// This is an implementation of the function `psync` in the paper [Brief Announcement: Preserving Happens-before in Persistent Memory](https://www.cs.rochester.edu/u/jhi1/papers/2016-spaa-transform) which provides a generic scheme for working with persistent memory and atomic operations.
///
/// Use this before taking any I/O action, issue a `persistent_sync()` to ensure all changes have reached persistent storage.
/// In practice this means using it before returning from any function call that has an observable side-effect, eg a constructor, `drop`, an enqueue or dequeue to a persistent queue, etc.
///
/// Acts as a memory fence.
///
/// Blocking.
///
/// Finishes when all preceding `persistent_fence()` in this thread have completed.
#[inline(always)]
pub fn persistent_sync()
{
	// Intel have withdrawn the `pcommit` instruction.
	// Originally this code would have been `sfence(); pcommit(); sfence()` for transient memory buffers; see table 1 in the paper [Brief Announcement: Preserving Happens-before in Persistent Memory](https://www.cs.rochester.edu/u/jhi1/papers/2016-spaa-transform).
	#[cfg(all(target_feature = "sse2", any(target_arch = "x86_64", target_arch = "x86")))] sfence();
	
	#[cfg(target_arch = "aarch64")] dmb_ish();
}
