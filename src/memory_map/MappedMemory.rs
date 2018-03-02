// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Mapped memory.
/// When dropped, un-maps the memory.
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MappedMemory(NonNull<u8>, usize, bool);

#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
impl Drop for MappedMemory
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { munmap(self.as_ptr() as *mut c_void, self.length()) };
	}
}

#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
impl MappedMemory
{
	/// Memory-mapped address.
	#[inline(always)]
	pub fn to_non_null(&self) -> NonNull<u8>
	{
		self.0
	}
	
	/// Memory-mapped pointer.
	#[inline(always)]
	pub fn as_ptr(&self) -> *mut u8
	{
		self.0.as_ptr()
	}
	
	/// Memory-mapped length.
	#[inline(always)]
	pub fn length(&self) -> usize
	{
		self.1
	}
	
	/// Was mapped with Linux 4.15+ `MAP_SYNC` flag.
	#[inline(always)]
	pub fn was_mapped_with_linux_sync_flag(&self) -> bool
	{
		self.2
	}
}
