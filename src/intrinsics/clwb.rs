// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Writes back to memory the cache line (if modified) that contains the linear address specified with the `address` parameter from any level of the cache hierarchy in the cache coherence domain.
/// The line may be retained in the cache hierarchy in non-modified state.
/// Retaining the line in the cache hierarchy is a performance optimization (treated as a hint by hardware) to reduce the possibility of cache miss on a subsequent access.
/// Hardware may choose to retain the line at any of the levels in the cache hierarchy, and in some cases, may invalidate the line from the cache hierarchy.
///
/// The `address` parameter is a byte memory location.
///
/// The `clwb` instruction is ordered only by store-fencing operations.
/// For example, software can use an `sfence`, `mfence`, `xchg` (eg CAS), or `LOCK`-prefixed instructions (eg fetch-add) to ensure that previous stores are included in the write-back.
/// The `clwb` instruction need not be ordered by another `clwb` or `clflushopt` instruction.
/// `clwb` is implicitly ordered with older stores executed by the logical processor to the same address.
/// For usages that require only writing back modified data from cache lines to memory (do not require the line to be invalidated), and expect to subsequently access the data, software is recommended to use `clwb` (with appropriate fencing) instead of `clflushopt` or `clflush` for improved performance.
///
/// To ensure this intrinsic is available, use `cargo rustc -- -C target-feature=+clwb`.
///
/// This intrinsic is not available for Intel architectures before Cannonlake or Icelake.
///
/// Executing `clwb` will nearly always cause a TSX abort.
//noinspection SpellCheckingInspection
#[allow(unused_variables)]
#[inline(always)]
pub fn clwb(address: *mut u8)
{
	// There is a LLVM intrinsic for `clwb` (`llvm.x86.clwb`) but it may not be exposed by Rust.
	
	#[cfg(all(target_feature = "clwb", any(target_arch = "x86_64", target_arch = "x86")))]
	unsafe
	{
		asm!
		(
			".byte 0x66; xsaveopt %0"
			:
				"+m" (*address)
			:
			:
			:
				"volatile"
		)
	}
}
