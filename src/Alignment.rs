// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Extension trait to align pointers and memory.
pub trait Alignment
{
	/// Rounds up to `alignment`.
	/// If already at `alignment` does nothing.
	/// `alignment_in_bytes` must be a power of two (and so can not be zero).
	#[inline(always)]
	fn round_up_to_alignment(self, alignment_in_bytes: usize) -> Self;
}

impl Alignment for usize
{
	#[inline(always)]
	fn round_up_to_alignment(self, alignment_in_bytes: usize) -> Self
	{
		debug_assert!(alignment_in_bytes.is_power_of_two(), "alignment_in_bytes must be a power of two");
		
		((self + (alignment_in_bytes - 1)) / alignment_in_bytes) * alignment_in_bytes
	}
}

impl Alignment for *mut u8
{
	#[inline(always)]
	fn round_up_to_alignment(self, alignment: usize) -> Self
	{
		(self as usize).round_up_to_alignment(alignment) as *mut u8
	}
}

