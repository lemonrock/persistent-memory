// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn flush_struct<T>(address: &T)
{
	flush_memory(address as *const T as *mut T as *mut c_void, size_of::<T>())
}

#[inline(always)]
pub(crate) fn flush_non_null<T>(address: NonNull<T>)
{
	flush_memory(address.as_ptr() as *mut c_void, size_of::<T>())
}

#[inline(always)]
pub(crate) fn flush_memory(address: *mut c_void, length: usize)
{
	unimplemented!();
}
