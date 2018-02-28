// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


// TODO: AtomicU64Pair (128-bit CAS)

/// A trait that integrates persistent memory with a Rust Atomic.
pub trait AtomicPersistentMemory<Value: Copy>
{
	/// A relaxed load for persistent memory.
	#[inline(always)]
	fn persistent_load_relaxed(&self) -> Value
	{
		self.load_(Relaxed)
	}
	
	/// An acquire load for persistent memory.
	#[inline(always)]
	fn persistent_load_acquire(&self) -> Value
	{
		let value = self.load_(Acquire);
		self.persistent_write_back_();
		persistent_fence();
		value
	}
	
	/// A sequentially consistent load for persistent memory.
	#[inline(always)]
	fn persistent_load_sequentially_consistent(&self) -> Value
	{
		let value = self.load_(SeqCst);
		self.persistent_write_back_();
		persistent_fence();
		value
	}
	
	/// A relaxed store for persistent memory.
	#[inline(always)]
	fn persistent_store_relaxed(&self, value: Value)
	{
		self.store_(value, Relaxed);
		self.persistent_write_back_()
	}
	
	/// A release store for persistent memory.
	#[inline(always)]
	fn persistent_store_release(&self, value: Value)
	{
		persistent_fence();
		self.store_(value, Release);
		self.persistent_write_back_()
	}
	
	/// A sequentially consistent store for persistent memory.
	#[inline(always)]
	fn persistent_store_sequentially_consistent(&self, value: Value)
	{
		persistent_fence();
		self.store_(value, SeqCst);
		self.persistent_write_back_()
	}
	
	/// A relaxed swap for persistent memory.
	#[inline(always)]
	fn persistent_swap_relaxed(&self, value: Value) -> Value
	{
		self.swap_(value, Relaxed)
	}
	
	/// A release swap for persistent memory.
	#[inline(always)]
	fn persistent_swap_release(&self, value: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let value = self.swap_(value, Release);
		locked_read_modify_write_operation_persistent_fence();
		value
	}
	
	/// An acquire swap for persistent memory.
	#[inline(always)]
	fn persistent_swap_acquire(&self, value: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let value = self.swap_(value, Acquire);
		locked_read_modify_write_operation_persistent_fence();
		value
	}
	
	/// An acquire-release swap for persistent memory.
	#[inline(always)]
	fn persistent_swap_acquire_release(&self, value: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let value = self.swap_(value, AcqRel);
		locked_read_modify_write_operation_persistent_fence();
		value
	}
	
	/// A sequentially consistent swap for persistent memory.
	#[inline(always)]
	fn persistent_swap_sequentially_consistent(&self, value: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let value = self.swap_(value, SeqCst);
		locked_read_modify_write_operation_persistent_fence();
		value
	}
	
	/// A relaxed-relaxed strong compare-and-swap for persistent memory.
	/// No weak variant is provided as it makes very little sense when adding in persistent behaviour.
	#[inline(always)]
	fn persistent_compare_and_swap_strong_relaxed_relaxed(&self, current: Value, new: Value) -> Result<Value, Value>
	{
		self.compare_exchange_(current, new, Relaxed, Relaxed)
	}
	
	/// A acquire-relaxed strong compare-and-swap for persistent memory.
	/// No weak variant is provided as it makes very little sense when adding in persistent behaviour.
	#[inline(always)]
	fn persistent_compare_and_swap_strong_acquire_relaxed(&self, current: Value, new: Value)-> Result<Value, Value>
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.compare_exchange_(current, new, Acquire, Relaxed);
		if result.is_ok()
		{
			locked_read_modify_write_operation_persistent_fence();
		}
		result
	}
	
	/// An acquire-acquire strong compare-and-swap for persistent memory.
	/// No weak variant is provided as it makes very little sense when adding in persistent behaviour.
	#[inline(always)]
	fn persistent_compare_and_swap_strong_acquire_acquire(&self, current: Value, new: Value)-> Result<Value, Value>
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.compare_exchange_(current, new, Acquire, Acquire);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// An acquire-release strong compare-and-swap for persistent memory.
	/// No weak variant is provided as it makes very little sense when adding in persistent behaviour.
	#[inline(always)]
	fn persistent_compare_and_swap_strong_acquire_release(&self, current: Value, new: Value)-> Result<Value, Value>
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.compare_exchange_(current, new, AcqRel, Acquire);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// A sequentially consistent strong compare-and-swap for persistent memory.
	/// No weak variant is provided as it makes very little sense when adding in persistent behaviour.
	#[inline(always)]
	fn persistent_compare_and_swap_strong_sequentially_consistent(&self, current: Value, new: Value)-> Result<Value, Value>
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.compare_exchange_(current, new, SeqCst, SeqCst);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn persistent_write_back_(&self)
	{
		persistent_write_back(self as *const Self as *mut Self as *mut u8)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn load_(&self, ordering: Ordering) -> Value;
	
	#[doc(hidden)]
	#[inline(always)]
	fn store_(&self, value: Value, ordering: Ordering);
	
	#[doc(hidden)]
	#[inline(always)]
	fn swap_(&self, value: Value, ordering: Ordering) -> Value;
	
	#[doc(hidden)]
	#[inline(always)]
	fn compare_exchange_(&self, current: Value, new: Value, success: Ordering, failure: Ordering) -> Result<Value, Value>;
}

macro_rules! atomic_persistent_memory_implementation
{
	($atomic_type:ident, $int_type:ident) =>
	{
		impl AtomicPersistentMemory<$int_type> for $atomic_type
		{
			#[inline(always)]
			fn load_(&self, ordering: Ordering) -> $int_type
			{
				self.load(ordering)
			}
			
			#[inline(always)]
			fn store_(&self, value: $int_type, ordering: Ordering)
			{
				self.store(value, ordering)
			}
			
			#[inline(always)]
			fn swap_(&self, value: $int_type, ordering: Ordering) -> $int_type
			{
				self.swap(value, ordering)
			}
			
			#[inline(always)]
			fn compare_exchange_(&self, current: $int_type, new: $int_type, success: Ordering, failure: Ordering) -> Result<$int_type, $int_type>
			{
				self.compare_exchange(current, new, success, failure)
			}
		}
	}
}

atomic_persistent_memory_implementation!(AtomicBool, bool);
atomic_persistent_memory_implementation!(AtomicI8, i8);
atomic_persistent_memory_implementation!(AtomicI16, i16);
atomic_persistent_memory_implementation!(AtomicI32, i32);
atomic_persistent_memory_implementation!(AtomicI64, i64);
atomic_persistent_memory_implementation!(AtomicIsize, isize);
atomic_persistent_memory_implementation!(AtomicU8, u8);
atomic_persistent_memory_implementation!(AtomicU16, u16);
atomic_persistent_memory_implementation!(AtomicU32, u32);
atomic_persistent_memory_implementation!(AtomicU64, u64);
atomic_persistent_memory_implementation!(AtomicUsize, usize);

impl<T> AtomicPersistentMemory<*mut T> for AtomicPtr<T>
{
	#[inline(always)]
	fn load_(&self, ordering: Ordering) -> *mut T
	{
		self.load(ordering)
	}
	
	#[inline(always)]
	fn store_(&self, value: *mut T, ordering: Ordering)
	{
		self.store(value, ordering)
	}
	
	#[inline(always)]
	fn swap_(&self, value: *mut T, ordering: Ordering) -> *mut T
	{
		self.swap(value, ordering)
	}
	
	#[inline(always)]
	fn compare_exchange_(&self, current: *mut T, new: *mut T, success: Ordering, failure: Ordering) -> Result<*mut T, *mut T>
	{
		self.compare_exchange(current, new, success, failure)
	}
}
