// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// An extension trait to provide methods to file paths used as memory files, such as DAX devices.
pub trait PersistentMemoryFilePathExt
{
	/// Persistent memory file size for use with `mmap()`.
	#[inline(always)]
	fn memory_file_size_for_use_with_mmap(&self) -> Result<u64, CouldNotObtainDaxDeviceStatisticError>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn file_size(&self) -> Result<u64, CouldNotObtainDaxDeviceStatisticError>;
}

impl PersistentMemoryFilePathExt for Path
{
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn memory_file_size_for_use_with_mmap(&self) -> Result<u64, CouldNotObtainDaxDeviceStatisticError>
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
	fn memory_file_size_for_use_with_mmap(&self) -> Result<usize, CouldNotObtainDaxDeviceStatisticError>
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
