// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Different memory address options for memory mapping.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MemoryMapAddress
{
	/// Ask the kernel to allocate a random address.
	Random,
	
	/// Try to find an address greater than or equal to the `minimum_address`, but don't fail to memory map if this address isn't used.
	Hint
	{
		/// Minimum address hint.
		/// Can be null (zero).
		minimum_address: *mut u8,
	},
	
	/// Memory map at this address and no other; will round it up to satisfy alignment.
	/// Memory mapping will fail if it can not be allocated at this address.
	Mandatory
	{
		/// Unaligned address to use.
		unaligned_address: *mut u8,
	},
}

impl MemoryMapAddress
{
	/// Returns an address and modified mmap flags to use with `mmap`.
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	pub fn address(&self, size: usize, alignment: usize, mmap_flags: MMapFlags) -> Result<(*mut c_void, MMapFlags), OccupiedMemoryMapParseError>
	{
		debug_assert!(alignment.is_power_of_two(), "alignment must be a power of two");
		
		use self::MemoryMapAddress::*;
		
		match *self
		{
			Random => Ok((null_mut(), mmap_flags)),
			
			Hint { minimum_address } =>
			{
				let address = match find_lowest_unoccupied_address_in_process_map(minimum_address, size, alignment)?
				{
					None => null_mut(),
					Some(address) => address as *mut c_void,
				};
				
				Ok((address, mmap_flags))
			}
			
			Mandatory { unaligned_address } =>
			{
				#[cfg(any(target_os = "android", target_os = "linux"))] let flags = mmap_flags | MAP_FIXED;
				#[cfg(target_os = "freebsd")] let flags = mmap_flags | MAP_FIXED | MAP_EXCL;
				
				let aligned_address = unaligned_address.round_up_to_alignment(alignment);
				
				Ok((aligned_address as *mut c_void, flags))
			}
		}
	}
}
