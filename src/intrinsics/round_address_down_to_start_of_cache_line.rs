// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Aligns an address to the start of a cache line by rounding it down.
#[inline(always)]
pub fn round_address_down_to_start_of_cache_line(address: *mut u8) -> *mut u8
{
	debug_assert_ne!(CacheLineSize, 0, "CacheLineSize can not be zero");
	
	const CacheLineFlags: usize = !(CacheLineSize - 1);
	
	((address as usize) & CacheLineFlags) as *mut u8
}
