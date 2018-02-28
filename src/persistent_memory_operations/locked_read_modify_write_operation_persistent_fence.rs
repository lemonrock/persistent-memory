// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Pedro Ramalhete & Andreia Correia argue that the preceding and following `persistent_fence()` for locked read-modify-write operations (eg compare-and-swap, swap, fetch-add, etc) are not needed on x86_64 when using `clflushopt()` or `clwb()` because locked read-modify-write instructions ensure order for `clflushopt()` and `clwb()`.
/// This implies that are needed when using the older `clflush()` and on AArch64.
#[inline(always)]
pub fn locked_read_modify_write_operation_persistent_fence()
{
	#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
	{
		if cfg!(target_feature = "clwb")
		{
			// Do nothing
		}
		else if cfg!(target_feature = "clflushopt")
		{
			// Do nothing
		}
		else if cfg!(target_feature = "sse2")
		{
			persistent_fence();
		}
	}
	
	#[cfg(target_arch = "aarch64")] persistent_fence();
}
