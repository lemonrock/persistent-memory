// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Returns `Ok(Some(address))` if a suitably aligned unoccupied address could be found.
/// Returns `Ok(None)` if no unoccupied address could be found.
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
#[inline(always)]
pub fn find_lowest_unoccupied_address_in_process_map(minimum_address_hint: *mut u8, size: usize, alignment: usize) -> Result<Option<*mut u8>, CouldNotMemoryMapError>
{
	debug_assert!(alignment.is_power_of_two(), "alignment must be a power of two");
	
	let occupied_memory_ranges = occupied_memory_ranges()?;
	
	let stack_starts_at = match occupied_memory_ranges.iter().next_back()
	{
		None => return Err(CouldNotMemoryMapError::InvalidLineInProcMap("There is no [stack] line")),
		Some((stack_starts_at, _stack_ends_at)) => *stack_starts_at,
	};
	
	let mut try_unaligned_allocation_starting_from = if minimum_address_hint.is_null()
	{
		memory_map_page_size()
	}
	else
	{
		minimum_address_hint as usize
	};
	
	loop
	{
		let aligned_allocation_would_start_at = try_unaligned_allocation_starting_from.round_up_to_alignment(alignment);
		let aligned_allocation_would_end_at = aligned_allocation_would_start_at + size;
		
		if aligned_allocation_would_end_at > stack_starts_at
		{
			return Ok(None);
		}
		
		if let Some((something_starts_at, something_ends_at)) = occupied_memory_ranges.range(try_unaligned_allocation_starting_from .. aligned_allocation_would_end_at).next_back()
		{
			// This check is to avoid a potential infinite loop bug which occurs if the range is empty (a zero-length allocation)
			if something_ends_at != something_starts_at
			{
				// Common case.
				try_unaligned_allocation_starting_from = *something_ends_at;
			}
			else
			{
				// Unusual case; force forward.
				try_unaligned_allocation_starting_from += 1;
			}
		}
		else
		{
			// Nothing in range... so it's all unoccupied memory.
			return Ok(Some(aligned_allocation_would_start_at as *mut u8))
		}
	}
}

#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
#[inline(always)]
fn occupied_memory_ranges() -> Result<BTreeMap<usize, usize>, CouldNotMemoryMapError>
{
	// Whilst it seems that CurrentProcessMapFile is sorted and has no overlapping entries, experience of Linux kernel /proc and /sys suggests that documentation is poor and breaking change high.
	let mut occupied_memory_ranges = BTreeMap::new();
	parse_current_process_map_file(|low, high|
	{
		if occupied_memory_ranges.range(low .. high).count() != 0
		{
			return Err(CouldNotMemoryMapError::InvalidLineInProcMap("low .. high already present"))
		}
		
		if let Some((_previous_low, previous_high)) = occupied_memory_ranges.range( .. low).next_back()
		{
			if *previous_high > low
			{
				return Err(CouldNotMemoryMapError::InvalidLineInProcMap("low .. high overlaps"))
			}
		}
		
		occupied_memory_ranges.insert(low, high);
		
		Ok(())
	})?;
	
	Ok(occupied_memory_ranges)
}

#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
#[inline(always)]
fn parse_current_process_map_file<MappedMemoryRangeUser: FnMut(usize, usize) -> Result<(), CouldNotMemoryMapError>>(mut mapped_memory_range_user: MappedMemoryRangeUser) -> Result<(), CouldNotMemoryMapError>
{
	#[cfg(any(target_os = "android", target_os = "linux"))] const CurrentProcessMapFile: &'static str = "/proc/self/maps";
	//noinspection SpellCheckingInspection
	#[cfg(target_os = "freebsd")] const CurrentProcessMapFile: &'static str = "/proc/curproc/map";
	let current_process_map_file = File::open(CurrentProcessMapFile)?;
	
	// NOTE: This could be done more efficiently using the `read!` macro in the `text_io` crate.
	let buffer_reader = BufReader::new(current_process_map_file);
	for line in buffer_reader.lines()
	{
		let line = line?;
		
		{
			// Sample line on Linux: `17add6e9000-17add7ae000 r-xp 00000000 08:03 2228226                      /bin/busybox`
			let low: String;
			let high: String;
			let _remainder: String;
			
			#[cfg(any(target_os = "android", target_os = "linux"))] try_scan!(line.bytes() => "{}-{} {}", low, high, _remainder);
			#[cfg(target_os = "freebsd")] try_scan!(line.bytes() => "{} {} {}", low, high, _remainder);
			
			let low = usize::from_str_radix(&low, 16)?;
			let high = usize::from_str_radix(&high, 16)?;
			
			// It might be possible for low == high for a zero-length mapping.
			if low > high
			{
				return Err(CouldNotMemoryMapError::InvalidLineInProcMap("low is greater than high"))
			}
			
			mapped_memory_range_user(low, high)?
		}
	}
	
	Ok(())
}
