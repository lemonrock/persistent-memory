// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Tries to find an aligned address suitable for `mmap` without inspecting the process map.
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
pub(crate) fn find_random_memory_map_unoccupied_address(size: usize, alignment: usize) -> Result<*mut u8, ()>
{
	// Alignment should be at least PageSize, and really ought to be 2Mb / 1Gb
	// MAP_HUGE_2MB, MAP_HUGE_1GB (since Linux 3.8)
	// Used in conjunction with MAP_HUGETLB to select alternative hugetlb page sizes
	
	/*
	              More generally, the desired huge page size can be configured
              by encoding the base-2 logarithm of the desired page size in
              the six bits at the offset MAP_HUGE_SHIFT.  (A value of zero
              in this bit field provides the default huge page size; the
              default huge page size can be discovered vie the Hugepagesize
              field exposed by /proc/meminfo.)  Thus, the above two con‐
              stants are defined as:

                  #define MAP_HUGE_2MB    (21 << MAP_HUGE_SHIFT)
                  #define MAP_HUGE_1GB    (30 << MAP_HUGE_SHIFT)

              The range of huge page sizes that are supported by the system
              can be discovered by listing the subdirectories in /sys/ker‐
              nel/mm/hugepages.
              
              
              FreeBSD has  MAP_ALIGNED(n)  and   MAP_ALIGNED_SUPER
              	https://www.unix.com/man-page/FreeBSD/2/mmap/
	*/
	
	const NoAddressHint: *mut c_void = null_mut();
	const NoFileDescriptor: c_int = -1;
	const NoOffset: i64 = 0;
	
	// NOTE: `MAP_PRIVATE` with `PROT_READ` tries to force no over-commit accounting.
	let enough_size_to_adjust_for_alignment = size + alignment;
	let address = unsafe { mmap(NoAddressHint, enough_size_to_adjust_for_alignment, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS, NoFileDescriptor, NoOffset) };
	if address == MAP_FAILED
	{
		Err(())
	}
	else
	{
		unsafe { munmap(address, enough_size_to_adjust_for_alignment) };
		
		let address = address as *mut u8;
		Ok(address.round_up_to_alignment(alignment))
	}
}
