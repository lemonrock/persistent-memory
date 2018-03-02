// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Returns a (cached) memory map page size.
#[cfg(unix)]
#[inline(always)]
pub fn memory_map_page_size() -> usize
{
	const UnassignedPageSize: usize = 0;
	
	static mut PageSize: usize = UnassignedPageSize;
	
	let page_size = unsafe { PageSize };
	
	// Technically, we ought to use an Atomic compare-and-swap, but the cost isn't worthwhile.
	if page_size == UnassignedPageSize
	{
		let page_size = unsafe { sysconf(_SC_PAGESIZE) } as usize;
		debug_assert_ne!(page_size, UnassignedPageSize, "page_size is UnassignedPageSize");
		unsafe { PageSize = page_size };
		page_size
	}
	else
	{
		page_size
	}
}
