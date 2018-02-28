// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


//noinspection SpellCheckingInspection
/// Invalidates from every level of the cache hierarchy in the cache coherence domain the cache line that contains the linear address specified with the `address` parameter.
/// If that cache line contains modified data at any level of the cache hierarchy, that data is written back to memory.
///
/// The `address` parameter is a byte memory location.
///
/// Executions of the `clflushopt` instruction are ordered with respect to fence instructions and to locked read-modify-write instructions; they are also ordered with respect to the following accesses to the cache line being invalidated: writes, executions of `clflush`, and executions of `clflushopt`.
/// They are not ordered with respect to writes, executions of `clflush`, or executions of `clflushopt` that access other cache lines; to enforce ordering with such an operation, software can insert an `sfence` instruction between `clflush` and that operation.
///
/// To ensure this intrinsic is available, use `cargo rustc -- -C target-feature=+clflushopt`.
///
/// This intrinsic is not available for Intel architectures before Skylake and AMD architectures before Zen v1.
///
/// Executing `clflushopt` will nearly always a TSX abort.
#[allow(unused_variables)]
#[inline(always)]
pub fn clflushopt(address: *mut u8)
{
	#[cfg(all(target_feature = "clflushopt", any(target_arch = "x86_64", target_arch = "x86")))]
	{
		extern
		{
			#[link_name = "llvm.x86.clflushopt"]
			pub fn llvm_x86_clflushopt(a: *mut i8) -> ();
		}
		
		unsafe { llvm_x86_clflushopt(address as *mut i8) }
		
		/*
			asm!
			(
				".byte 0x66; clflush %0"
				:
					"+m" (address)
				:
				:
				:
					"volatile"
			);
		*/
	}
}
