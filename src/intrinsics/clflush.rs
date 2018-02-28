// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


//noinspection SpellCheckingInspection
/// Invalidates from every level of the cache hierarchy in the cache coherence domain the cache line that contains the linear address specified with the `address` parameter.
/// If that cache line contains modified data at any level of the cache hierarchy, that data is written back to memory.
/// The `address` parameter is a byte memory location.
/// Executions of the `clflush` instruction are ordered with respect to each other and with respect to writes, locked read-modify-write instructions, fence instructions, and executions of `clflushopt` to the same cache line.
/// They are not ordered with respect to executions of `clflushopt` to different cache lines.
///
/// To ensure this intrinsic is available, use `cargo rustc -- -C target-feature=+sse2`. (Technically `clflush` is not part of SSE2 but clang doesn't seem to have a separate feature for it).
///
/// It is not known if executing `clflush` will nearly always cause a TSX abort, but it should be assumed that it does.
#[allow(unused_variables)]
#[inline(always)]
pub fn clflush(address: *mut u8)
{
	#[cfg(all(target_feature = "sse2", any(target_arch = "x86_64", target_arch = "x86")))]
	{
		extern
		{
			#[link_name = "llvm.x86.clflush"]
			pub fn llvm_x86_clflush(a: *mut i8) -> ();
		}
		
		unsafe { llvm_x86_clflush(address as *mut i8) }
	}
}
