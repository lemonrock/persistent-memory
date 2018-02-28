// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


//noinspection SpellCheckingInspection
/// Issues a `sfence`.
/// Orders processor execution relative to all memory stores prior to the `sfence` instruction.
/// The processor ensures that every store prior to SFENCE is globally visible before any store after `sfence` becomes globally visible.
/// The `sfence` instruction is ordered with respect to memory stores, other `sfence` instructions, `mfence` instructions, and any serializing instructions (such as the `cpuid` instruction).
/// It is not ordered with respect to memory loads or the `lfence` instruction.
///
/// Weakly ordered memory types can be used to achieve higher processor performance through such techniques as out-of-order issue, write-combining, and write-collapsing.
/// The degree to which a consumer of data recognizes or knows that the data is weakly ordered varies among applications and may be unknown to the producer of this data.
/// The `sfence` instruction provides a performance-efficient way of ensuring store ordering between routines that produce weakly-ordered results and routines that consume this data.
///
/// To ensure this intrinsic is available, use `cargo rustc -- -C target-feature=+sse2`.
///
/// It is believed that `sfence` does not cause a TSX abort.
#[allow(unused_variables)]
#[inline(always)]
pub fn sfence()
{
	#[cfg(all(target_feature = "sse2", any(target_arch = "x86_64", target_arch = "x86")))]
	{
		extern "platform-intrinsic"
		{
			#[inline(always)]
			fn x86_mm_sfence();
		}
		
		unsafe { x86_mm_sfence() }
	}
}
