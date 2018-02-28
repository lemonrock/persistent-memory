// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A convenient way to use the methods on a cto pool.
trait PMEMctopoolExt
{
	/// Close the CTO pool.
	/// self can not be null.
	#[inline(always)]
	fn close(self);
	
	/// Get the root pointer which can be null.
	/// self can not be null.
	#[inline(always)]
	fn get_root<T>(self) -> *mut T;
	
	/// Set the root pointer. Should only be used if the current root pointer is null. Root pointer to set should not be null.
	/// Should be a pointer to an object previously created with one of the following methods on *mut PMEMctopool:-
	/// * `malloc()`
	/// * `aligned_alloc()`
	/// * `calloc()`
	/// * `strdup()`
	/// * `wcsdup()`
	/// The persistent object must eventually be free'd with our `free()`.
	/// self can not be null.
	#[inline(always)]
	fn set_root<T>(self, root: *mut T);
	
	/// The size_of::<T> must not be zero.
	/// If memory can not be allocated, returns a PmdkError with `isENOMEM()` true. Never returns `Ok(null_mut())`.
	/// self can not be null.
	#[inline(always)]
	fn malloc<T>(self) -> Result<*mut T, PmdkError>;
	
	/// Aligned allocation.
	/// If memory can not be allocated, returns a PmdkError with `isENOMEM()` true. Never returns `Ok(null_mut())`.
	/// self can not be null.
	#[inline(always)]
	fn aligned_alloc(self, alignment: usize, size: usize) -> Result<*mut c_void, PmdkError>;
	
	/// Pointer must not be null.
	/// self can not be null.
	#[inline(always)]
	fn usable_size(self, pointer: *mut c_void) -> size_t;
	
	/// Pointer must not be null.
	/// new_size can not be zero.
	/// If memory can not be allocated, returns a PmdkError with `isENOMEM()` true. Never returns `Ok(null_mut())`.
	/// self can not be null.
	#[inline(always)]
	fn realloc(self, pointer: *mut c_void, new_size: size_t) -> Result<*mut c_void, PmdkError>;
	
	/// Pointer must not be null.
	/// self can not be null.
	#[inline(always)]
	fn free<T>(self, pointer: *mut T);
}

impl PMEMctopoolExt for *mut PMEMctopool
{
	#[inline(always)]
	fn close(self)
	{
		debug_assert!(self.is_not_null(), "self can not be null");
		
		unimplemented!()
	}
	
	#[inline(always)]
	fn get_root<T>(self) -> *mut T
	{
		debug_assert!(self.is_not_null(), "self can not be null");
		
		unimplemented!()
	}
	
	#[inline(always)]
	fn set_root<T>(self, root: *mut T)
	{
		debug_assert!(self.is_not_null(), "self can not be null");
		debug_assert!(root.is_not_null(), "root can not be null");
		
		unimplemented!()
	}
	
	#[inline(always)]
	fn malloc<T>(self) -> Result<*mut T, PmdkError>
	{
		debug_assert!(self.is_not_null(), "self can not be null");
		
		let size = size_of::<T>() as size_t;
		debug_assert!(size != 0, "size_of::<T>() can not be zero");
		
		unimplemented!()
	}
	
	#[inline(always)]
	fn aligned_alloc(self, alignment: usize, size: usize) -> Result<*mut c_void, PmdkError>
	{
		#[inline(always)]
		fn is_power_of_two(value: size_t) -> bool
		{
			(value != 0) && ((value & (value - 1)) == 0)
		}
		
		debug_assert!(self.is_not_null(), "self can not be null");
		
		debug_assert!(!is_power_of_two(alignment), "alignment must be a power of two");
		
		debug_assert!(size != 0, "size_of::<T>() can not be zero");
		
		unimplemented!()
	}
	
	#[inline(always)]
	fn usable_size(self, pointer: *mut c_void) -> size_t
	{
		debug_assert!(self.is_not_null(), "self can not be null");
		debug_assert!(pointer.is_not_null(), "pointer can not be null");
		
		unimplemented!()
	}
	
	#[inline(always)]
	fn realloc(self, pointer: *mut c_void, new_size: size_t) -> Result<*mut c_void, PmdkError>
	{
		debug_assert!(self.is_not_null(), "self can not be null");
		debug_assert!(pointer.is_not_null(), "pointer can not be null");
		debug_assert!(new_size != 0, "new_size can not be zero");
		
		unimplemented!()
	}
	
	#[inline(always)]
	fn free<T>(self, pointer: *mut T)
	{
		debug_assert!(self.is_not_null(), "self can not be null");
		debug_assert!(pointer.is_not_null(), "pointer can not be null");
		
		unimplemented!()
	}
}
