// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// An extension trait to provide methods to file paths used as memory files, such as DAX devices.
pub trait PersistentMemoryFilePathExt
{
	/// memory map (`mmap`) persistent memory file
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn memory_map(&self, read_only: bool, alignment: Option<usize>) -> Result<*mut u8, CouldNotMemoryMapError>;
	
	/// Persistent memory file size for use with `mmap()`.
	#[inline(always)]
	fn memory_file_size_for_use_with_memory_map(&self) -> Result<u64, CouldNotObtainDaxDeviceStatisticError>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn file_size(&self) -> Result<u64, CouldNotObtainDaxDeviceStatisticError>;
}

fn util_map_hint(size: u64, alignment: usize) -> Option<*mut u8>
{
	unimplemented!();
}

use ::std::ptr::null_mut;

impl PersistentMemoryFilePathExt for Path
{
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn memory_map(&self, read_only: bool, alignment: Option<usize>) -> Result<*mut u8, CouldNotMemoryMapError>
	{
//		use ::libc::mmap;
//		use ::libc::MAP_SHARED;
		use ::libc::PROT_READ;
		use ::libc::PROT_WRITE;
		
		
		let size = self.memory_file_size_for_use_with_memory_map()?;
		
		let protection = if read_only
		{
			PROT_READ
		}
		else
		{
			PROT_READ | PROT_WRITE
		};
		
		let address_hint: *mut u8 = if let Some(alignment) = alignment
		{
			let address_hint = util_map_hint(size, alignment);
			if address_hint.is_none()
			{
				return Err(CouldNotMemoryMapError::CouldNotFindAContiguousRegionToMemoryMapInto(size, alignment));
			}
			else
			{
				address_hint.unwrap()
			}
		}
		else
		{
			null_mut()
		};
		
		let address = null_mut(); //util_map_sync(address_hint, size, protection, flags, 0, map_sync_ptr) as *mut u8;
		
		if address.is_null()
		{
			Err(CouldNotMemoryMapError::MMapFailed)
		}
		else
		{
			Ok(address)
		}
		
		/*
			#ifndef MAP_SYNC
			#define MAP_SYNC 0x80000
			#endif
			
			#ifndef MAP_SHARED_VALIDATE
			#define MAP_SHARED_VALIDATE 0x03
			#endif
			
			 // util_map_sync -- memory map given file into memory, if MAP_SHARED flag is
			 // provided it attempts to use MAP_SYNC flag. Otherwise it fallbacks to
			 // mmap(2).
			void *
			util_map_sync(void *addr, size_t len, int proto, int flags, int fd,
				os_off_t offset, int *map_sync)
			{
				LOG(15, "addr %p len %zu proto %x flags %x fd %d offset %ld "
					"map_sync %p", addr, len, proto, flags, fd, offset, map_sync);
			
				if (map_sync)
					*map_sync = 0;
			
				/* if map_sync is NULL do not even try to mmap with MAP_SYNC flag */
				if (!map_sync || flags & MAP_PRIVATE)
					return mmap(addr, len, proto, flags, fd, offset);
			
				/* MAP_SHARED */
				void *ret = mmap(addr, len, proto,
						flags | MAP_SHARED_VALIDATE | MAP_SYNC,
						fd, offset);
				if (ret != MAP_FAILED) {
					LOG(4, "mmap with MAP_SYNC succeeded");
					*map_sync = 1;
					return ret;
				}
			
				if (errno == EINVAL || errno == ENOTSUP) {
					LOG(4, "mmap with MAP_SYNC not supported");
					return mmap(addr, len, proto, flags, fd, offset);
				}
			
				/* other error */
				return MAP_FAILED;
			}
			

/*
 * util_map_hint -- determine hint address for mmap()
 *
 * If PMEM_MMAP_HINT environment variable is not set, we let the system to pick
 * the randomized mapping address.  Otherwise, a user-defined hint address
 * is used.
 *
 * ALSR in 64-bit Linux kernel uses 28-bit of randomness for mmap
 * (bit positions 12-39), which means the base mapping address is randomized
 * within [0..1024GB] range, with 4KB granularity.  Assuming additional
 * 1GB alignment, it results in 1024 possible locations.
 *
 * Configuring the hint address via PMEM_MMAP_HINT environment variable
 * disables address randomization.  In such case, the function will search for
 * the first unused, properly aligned region of given size, above the specified
 * address.
 */
char *
util_map_hint(size_t len, size_t req_align)
{
	LOG(3, "len %zu req_align %zu", len, req_align);

	char *hint_addr = MAP_FAILED;

	/* choose the desired alignment based on the requested length */
	size_t align = util_map_hint_align(len, req_align);

	if (Mmap_no_random) {
		LOG(4, "user-defined hint %p", (void *)Mmap_hint);
		hint_addr = util_map_hint_unused((void *)Mmap_hint, len, align);
	} else {
		/*
		 * Create dummy mapping to find an unused region of given size.
		 * Request for increased size for later address alignment.
		 * Use MAP_PRIVATE with read-only access to simulate
		 * zero cost for overcommit accounting.  Note: MAP_NORESERVE
		 * flag is ignored if overcommit is disabled (mode 2).
		 */
		char *addr = mmap(NULL, len + align, PROT_READ,
					MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
		if (addr != MAP_FAILED) {
			LOG(4, "system choice %p", addr);
			hint_addr = (char *)roundup((uintptr_t)addr, align);
			munmap(addr, len + align);
		}
	}
	LOG(4, "hint %p", hint_addr);

	return hint_addr;
}
		*/
		
		
		
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
