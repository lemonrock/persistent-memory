// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// A trait that integrates persistent memory operations on integers (eg `usize`) with a Rust Atomic.
pub trait IntegerAtomicPersistentMemory<Value: Copy>
{
	/// A relaxed fetch-add for persistent memory.
	#[inline(always)]
	fn persistent_fetch_add_relaxed(&self, increment: Value) -> Value
	{
		self.fetch_add_(increment, Relaxed)
	}
	
	/// A release fetch-add for persistent memory.
	#[inline(always)]
	fn persistent_fetch_add_release(&self, increment: Value) -> Value
	{
		persistent_fence();
		let result = self.fetch_add_(increment, Release);
		persistent_fence();
		result
	}
	
	/// An acquire fetch-add for persistent memory.
	#[inline(always)]
	fn persistent_fetch_add_acquire(&self, increment: Value) -> Value
	{
		persistent_fence();
		let result = self.fetch_add_(increment, Acquire);
		persistent_fence();
		result
	}
	
	/// An acquire-release fetch-add for persistent memory.
	#[inline(always)]
	fn persistent_fetch_add_acquire_release(&self, increment: Value) -> Value
	{
		persistent_fence();
		let result = self.fetch_add_(increment, AcqRel);
		persistent_fence();
		result
	}
	
	/// A sequentially consistent fetch-add for persistent memory.
	#[inline(always)]
	fn persistent_fetch_add_sequentially_consistent(&self, increment: Value) -> Value
	{
		persistent_fence();
		let result = self.fetch_add_(increment, SeqCst);
		persistent_fence();
		result
	}
	
	/// A relaxed fetch-sub for persistent memory.
	#[inline(always)]
	fn persistent_fetch_sub_relaxed(&self, increment: Value) -> Value
	{
		self.fetch_sub_(increment, Relaxed)
	}
	
	/// A release fetch-sub for persistent memory.
	#[inline(always)]
	fn persistent_fetch_sub_release(&self, increment: Value) -> Value
	{
		persistent_fence();
		let result = self.fetch_sub_(increment, Release);
		persistent_fence();
		result
	}
	
	/// An acquire fetch-sub for persistent memory.
	#[inline(always)]
	fn persistent_fetch_sub_acquire(&self, increment: Value) -> Value
	{
		persistent_fence();
		let result = self.fetch_sub_(increment, Acquire);
		persistent_fence();
		result
	}
	
	/// An acquire-release fetch-sub for persistent memory.
	#[inline(always)]
	fn persistent_fetch_sub_acquire_release(&self, increment: Value) -> Value
	{
		persistent_fence();
		let result = self.fetch_sub_(increment, AcqRel);
		persistent_fence();
		result
	}
	
	/// A sequentially consistent fetch-sub for persistent memory.
	#[inline(always)]
	fn persistent_fetch_sub_sequentially_consistent(&self, increment: Value) -> Value
	{
		persistent_fence();
		let result = self.fetch_sub_(increment, SeqCst);
		persistent_fence();
		result
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fetch_add_(&self, increment: Value, ordering: Ordering) -> Value;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fetch_sub_(&self, increment: Value, ordering: Ordering) -> Value;
}

macro_rules! integer_atomic_persistent_memory_implementation
{
	($atomic_type:ident, $int_type:ident) =>
	{
		impl IntegerAtomicPersistentMemory<$int_type> for $atomic_type
		{
			#[inline(always)]
			fn fetch_add_(&self, increment: $int_type, ordering: Ordering) -> $int_type
			{
				self.fetch_add(increment, ordering)
			}
			
			#[inline(always)]
			fn fetch_sub_(&self, increment: $int_type, ordering: Ordering) -> $int_type
			{
				self.fetch_sub(increment, ordering)
			}
		}
	}
}

integer_atomic_persistent_memory_implementation!(AtomicI8, i8);
integer_atomic_persistent_memory_implementation!(AtomicI16, i16);
integer_atomic_persistent_memory_implementation!(AtomicI32, i32);
integer_atomic_persistent_memory_implementation!(AtomicI64, i64);
integer_atomic_persistent_memory_implementation!(AtomicIsize, isize);
integer_atomic_persistent_memory_implementation!(AtomicU8, u8);
integer_atomic_persistent_memory_implementation!(AtomicU16, u16);
integer_atomic_persistent_memory_implementation!(AtomicU32, u32);
integer_atomic_persistent_memory_implementation!(AtomicU64, u64);
integer_atomic_persistent_memory_implementation!(AtomicUsize, usize);
