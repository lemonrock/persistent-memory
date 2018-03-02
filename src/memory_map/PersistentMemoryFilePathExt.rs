// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// An extension trait to provide methods to file paths used as memory files, such as DAX devices.
pub trait PersistentMemoryFilePathExt
{
	/// Memory map (`mmap`) persistent memory file.
	/// Returns pointer to mapped address and boolean indicating whether the Linux 4.15+ `MAP_SYNC` flag was used successfully.
	/// If a minimum_address_hint_and_alignment is supplied, then `MAP_FIXED` is used to try to fix the mapping at a particular location.
	/// `offset` should normally be zero.
	/// The returned address will need to be un-mapped with `munmap`.
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn memory_map(&self, read_only: bool, minimum_address_hint_and_alignment: Option<(*mut u8, usize)>, offset: u64) -> Result<(*mut u8, bool), CouldNotMemoryMapError>;
	
	/// Persistent memory file size for use with `mmap()`.
	#[inline(always)]
	fn memory_file_size_for_use_with_memory_map(&self) -> Result<u64, CouldNotObtainDaxDeviceStatisticError>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn file_size(&self) -> Result<u64, CouldNotObtainDaxDeviceStatisticError>;
}

impl PersistentMemoryFilePathExt for Path
{
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn memory_map(&self, read_only: bool, minimum_address_hint_and_alignment: Option<(*mut u8, usize)>, offset: u64) -> Result<(*mut u8, bool), CouldNotMemoryMapError>
	{
		let size = self.memory_file_size_for_use_with_memory_map()?;
		let size = (size - offset) as usize;
		
		let protection = if read_only
		{
			PROT_READ
		}
		else
		{
			PROT_READ | PROT_WRITE
		};
		
		let mut flags = MAP_SHARED;
		
		let address: *mut c_void = if let Some((minimum_address_hint, alignment)) = minimum_address_hint_and_alignment
		{
			debug_assert!(alignment.is_power_of_two(), "alignment must be a power of two");
			
			let address_hint = find_lowest_unoccupied_address_in_process_map(minimum_address_hint, size, alignment)?;
			if address_hint.is_none()
			{
				return Err(CouldNotMemoryMapError::CouldNotFindAContiguousRegionToMemoryMapInto(size as u64, alignment));
			}
			else
			{
				flags |= MAP_FIXED;
				
				address_hint.unwrap() as *mut c_void
			}
		}
		else
		{
			null_mut()
		};
		
		let mmap_offset = offset as i64;
		
		let memory_map_file = OpenOptions::new().read(true).write(true).open(self)?;
		let memory_map_file_descriptor = memory_map_file.as_raw_fd();
		
		// Linux 4.15 (28th January 2018) introduces the `MAP_SYNC` and `MAP_SHARED_VALIDATE` flags to `mmap(2)`, a mechanism that implements synchronous page faults for DAX mappings to make flushing of DAX mappings possible from userspace so that they can be flushed on finer than page granularity and also avoid the overhead of a syscall.
		// It arranges for any filesystem metadata updates that may be required to satisfy a write fault to also be flushed ("on disk") before the kernel returns to userspace from the fault handler.
		// Effectively every write-fault that dirties metadata completes an `fsync()` before returning from the fault handler.
		// The new `MAP_SHARED_VALIDATE` mapping type guarantees that the `MAP_SYNC` flag is validated as supported by the filesystem's `mmap()` implementation.
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			const MAP_SYNC: i32 = 0x80000;
			const MAP_SHARED_VALIDATE: i32 = 0x03;
			let new_linux_flags = flags | MAP_SHARED_VALIDATE | MAP_SYNC;
			
			let address = unsafe { mmap(address, size, protection, new_linux_flags, memory_map_file_descriptor, mmap_offset) };
			if address == MAP_FAILED
			{
				// Try again without MAP_SHARED_VALIDATE | MAP_SYNC as these are very new.
				let address = unsafe { mmap(address, size, protection, flags, memory_map_file_descriptor, mmap_offset) };
				if address == MAP_FAILED
				{
					Err(CouldNotMemoryMapError::MMapFailed)
				}
				else
				{
					Ok((address as *mut u8, false))
				}
			}
			else
			{
				Ok((address as *mut u8, true))
			}
		}
		
		#[cfg(not(any(target_os = "android", target_os = "linux")))]
		{
			let address = unsafe { mmap(address, size, protection, flags, memory_map_file_descriptor, mmap_offset) };
			if address == MAP_FAILED
			{
				Err(CouldNotMemoryMapError::MMapFailed)
			}
			else
			{
				Ok((address as *mut u8, false))
			}
		}
	}
	
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn memory_file_size_for_use_with_memory_map(&self) -> Result<u64, CouldNotObtainDaxDeviceStatisticError>
	{
		if self.is_this_a_dax_device()
		{
			self.find_dax_device_size().map(|size| size as u64)
		}
		else
		{
			self.file_size()
		}
	}
	
	#[cfg(not(any(target_os = "android", target_os = "freebsd", target_os = "linux")))]
	#[inline(always)]
	fn memory_file_size_for_use_with_memory_map(&self) -> Result<usize, CouldNotObtainDaxDeviceStatisticError>
	{
		self.file_size()
	}
	
	#[inline(always)]
	fn file_size(&self) -> Result<u64, CouldNotObtainDaxDeviceStatisticError>
	{
		let metadata = self.metadata()?;
		if metadata.is_file()
		{
			Ok(metadata.len())
		}
		else
		{
			Err(CouldNotObtainDaxDeviceStatisticError::IsNotAFile)
		}
	}
}
