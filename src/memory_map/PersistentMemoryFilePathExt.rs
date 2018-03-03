// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/*


use ::parking_lot::Mutex;
		// (1) `mmap` can overwrite an existing mapping.
		// (2) Finding a suitable unoccupied address is not thread safe.
		static GlobalMemoryMapLock: Mutex<()> = Mutex::new(());
		
*/

/// An extension trait to provide methods to file paths used as memory files, such as DAX devices.
pub trait PersistentMemoryFilePathExt
{
	/// Memory map (`mmap`) persistent memory file.
	/// Returns pointer to mapped address and boolean indicating whether the Linux 4.15+ `MAP_SYNC` flag was used successfully.
	/// If a minimum_address_hint_and_alignment is supplied, then `MAP_FIXED` is used to try to fix the mapping at a particular location.
	/// `offset` should normally be zero.
	///
	/// When the result is dropped the memory mapping will be un-mapped using `munmap`.
	///
	/// Memory mapping at fixed addresses is not thread-safe (`mmap` can silently replace existing mappings).
	///
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn memory_map(&self, read_only: bool, memory_map_address: MemoryMapAddress, offset: u64, alignment_if_not_dax_device: usize) -> Result<MappedMemory, CouldNotMemoryMapError>;
	
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
	fn memory_map(&self, read_only: bool, memory_map_address: MemoryMapAddress, offset: u64, alignment_if_not_dax_device: usize) -> Result<MappedMemory, CouldNotMemoryMapError>
	{
		let size = self.memory_file_size_for_use_with_memory_map()?;
		let size = (size - offset) as usize;
		
		let alignment = if self.is_this_a_dax_device()
		{
			self.find_dax_device_alignment()?
		}
		else
		{
			assert!(alignment_if_not_dax_device.is_power_of_two(), "alignment_if_not_dax_device '{}' is not a power of two", alignment_if_not_dax_device);
			assert!(alignment_if_not_dax_device >= memory_map_page_size(), "alignment_if_not_dax_device '{}' is less than memory_map_page_size '{}'", alignment_if_not_dax_device, memory_map_page_size());
			
			alignment_if_not_dax_device
		};
		
		let (address, mmap_flags) = memory_map_address.address(size, alignment, MAP_SHARED)?;
		
		let mmap_offset = offset as i64;
		
		let protection = if read_only
		{
			PROT_READ
		}
		else
		{
			PROT_READ | PROT_WRITE
		};
		let memory_map_file = OpenOptions::new().read(true).write(!read_only).open(self)?;
		let memory_map_file_descriptor = memory_map_file.as_raw_fd();
		
		// Linux 4.15 (28th January 2018) introduces the `MAP_SYNC` and `MAP_SHARED_VALIDATE` mmap_flags to `mmap(2)`, a mechanism that implements synchronous page faults for DAX mappings to make flushing of DAX mappings possible from userspace so that they can be flushed on finer than page granularity and also avoid the overhead of a syscall.
		// It arranges for any filesystem metadata updates that may be required to satisfy a write fault to also be flushed ("on disk") before the kernel returns to userspace from the fault handler.
		// Effectively every write-fault that dirties metadata completes an `fsync()` before returning from the fault handler.
		// The new `MAP_SHARED_VALIDATE` mapping type guarantees that the `MAP_SYNC` flag is validated as supported by the filesystem's `mmap()` implementation.
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			const MAP_SYNC: i32 = 0x80000;
			const MAP_SHARED_VALIDATE: i32 = 0x03;
			let new_linux_flags = mmap_flags | MAP_SHARED_VALIDATE | MAP_SYNC;
			
			let address = unsafe { mmap(address, size, protection, new_linux_flags, memory_map_file_descriptor, mmap_offset) };
			if address == MAP_FAILED
			{
				// Try again without MAP_SHARED_VALIDATE | MAP_SYNC in case the filesystem does not support them; mmap *does not fail* for invalid or unrecognised flags.
				let address = unsafe { mmap(address, size, protection, mmap_flags, memory_map_file_descriptor, mmap_offset) };
				if address == MAP_FAILED
				{
					Err(CouldNotMemoryMapError::MMapFailed)
				}
				else
				{
					Ok(MappedMemory((address as *mut u8).to_non_null(), size, false))
				}
			}
			else
			{
				Ok(MappedMemory((address as *mut u8).to_non_null(), size, true))
			}
		}
		
		#[cfg(not(any(target_os = "android", target_os = "linux")))]
		{
			let address = unsafe { mmap(address, size, protection, mmap_flags, memory_map_file_descriptor, mmap_offset) };
			if address == MAP_FAILED
			{
				Err(CouldNotMemoryMapError::MMapFailed)
			}
			else
			{
				Ok(MappedMemory((address as *mut u8).to_non_null(), size, false))
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
