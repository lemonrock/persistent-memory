// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// A trait that integrates persistent memory operations on bits (integers and booleans) (eg `usize` and `bool`) with a Rust Atomic.
pub trait BitAtomicPersistentMemory<Value: Copy>
{
	/// A relaxed fetch-and for persistent memory.
	#[inline(always)]
	fn persistent_fetch_and_relaxed(&self, increment: Value) -> Value
	{
		self.fetch_and_(increment, Relaxed)
	}
	
	/// A release fetch-and for persistent memory.
	#[inline(always)]
	fn persistent_fetch_and_release(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_and_(increment, Release);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// An acquire fetch-and for persistent memory.
	#[inline(always)]
	fn persistent_fetch_and_acquire(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_and_(increment, Acquire);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// An acquire-release fetch-and for persistent memory.
	#[inline(always)]
	fn persistent_fetch_and_acquire_release(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_and_(increment, AcqRel);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// A sequentially consistent fetch-and for persistent memory.
	#[inline(always)]
	fn persistent_fetch_and_sequentially_consistent(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_and_(increment, SeqCst);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// A relaxed fetch-or for persistent memory.
	#[inline(always)]
	fn persistent_fetch_or_relaxed(&self, increment: Value) -> Value
	{
		self.fetch_or_(increment, Relaxed)
	}
	
	/// A release fetch-or for persistent memory.
	#[inline(always)]
	fn persistent_fetch_or_release(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_or_(increment, Release);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// An acquire fetch-or for persistent memory.
	#[inline(always)]
	fn persistent_fetch_or_acquire(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_or_(increment, Acquire);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// An acquire-release fetch-or for persistent memory.
	#[inline(always)]
	fn persistent_fetch_or_acquire_release(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_or_(increment, AcqRel);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// A sequentially consistent fetch-or for persistent memory.
	#[inline(always)]
	fn persistent_fetch_or_sequentially_consistent(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_or_(increment, SeqCst);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// A relaxed fetch-xor for persistent memory.
	#[inline(always)]
	fn persistent_fetch_xor_relaxed(&self, increment: Value) -> Value
	{
		self.fetch_xor_(increment, Relaxed)
	}
	
	/// A release fetch-xor for persistent memory.
	#[inline(always)]
	fn persistent_fetch_xor_release(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_xor_(increment, Release);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// An acquire fetch-xor for persistent memory.
	#[inline(always)]
	fn persistent_fetch_xor_acquire(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_xor_(increment, Acquire);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// An acquire-release fetch-xor for persistent memory.
	#[inline(always)]
	fn persistent_fetch_xor_acquire_release(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_xor_(increment, AcqRel);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// A sequentially consistent fetch-xor for persistent memory.
	#[inline(always)]
	fn persistent_fetch_xor_sequentially_consistent(&self, increment: Value) -> Value
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_xor_(increment, SeqCst);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fetch_and_(&self, increment: Value, ordering: Ordering) -> Value;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fetch_or_(&self, increment: Value, ordering: Ordering) -> Value;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fetch_xor_(&self, increment: Value, ordering: Ordering) -> Value;
}

macro_rules! bit_atomic_persistent_memory_implementation
{
	($atomic_type:ident, $int_type:ident) =>
	{
		impl BitAtomicPersistentMemory<$int_type> for $atomic_type
		{
			#[inline(always)]
			fn fetch_and_(&self, increment: $int_type, ordering: Ordering) -> $int_type
			{
				self.fetch_and(increment, ordering)
			}
			
			#[inline(always)]
			fn fetch_or_(&self, increment: $int_type, ordering: Ordering) -> $int_type
			{
				self.fetch_or(increment, ordering)
			}
			
			#[inline(always)]
			fn fetch_xor_(&self, increment: $int_type, ordering: Ordering) -> $int_type
			{
				self.fetch_xor(increment, ordering)
			}
		}
	}
}

bit_atomic_persistent_memory_implementation!(AtomicBool, bool);
bit_atomic_persistent_memory_implementation!(AtomicI8, i8);
bit_atomic_persistent_memory_implementation!(AtomicI16, i16);
bit_atomic_persistent_memory_implementation!(AtomicI32, i32);
bit_atomic_persistent_memory_implementation!(AtomicI64, i64);
bit_atomic_persistent_memory_implementation!(AtomicIsize, isize);
bit_atomic_persistent_memory_implementation!(AtomicU8, u8);
bit_atomic_persistent_memory_implementation!(AtomicU16, u16);
bit_atomic_persistent_memory_implementation!(AtomicU32, u32);
bit_atomic_persistent_memory_implementation!(AtomicU64, u64);
bit_atomic_persistent_memory_implementation!(AtomicUsize, usize);
