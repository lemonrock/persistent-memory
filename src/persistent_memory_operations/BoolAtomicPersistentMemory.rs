// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// A trait that integrates persistent memory operations on a bool with a Rust AtomicBool.
pub trait BoolAtomicPersistentMemory: BitAtomicPersistentMemory<bool>
{
	/// A relaxed fetch-and for persistent memory.
	#[inline(always)]
	fn persistent_fetch_nand_relaxed(&self, increment: bool) -> bool
	{
		self.fetch_nand_(increment, Relaxed)
	}
	
	/// A release fetch-and for persistent memory.
	#[inline(always)]
	fn persistent_fetch_nand_release(&self, increment: bool) -> bool
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_nand_(increment, Release);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// An acquire fetch-and for persistent memory.
	#[inline(always)]
	fn persistent_fetch_nand_acquire(&self, increment: bool) -> bool
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_nand_(increment, Acquire);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// An acquire-release fetch-and for persistent memory.
	#[inline(always)]
	fn persistent_fetch_nand_acquire_release(&self, increment: bool) -> bool
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_nand_(increment, AcqRel);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	/// A sequentially consistent fetch-and for persistent memory.
	#[inline(always)]
	fn persistent_fetch_nand_sequentially_consistent(&self, increment: bool) -> bool
	{
		locked_read_modify_write_operation_persistent_fence();
		let result = self.fetch_nand_(increment, SeqCst);
		locked_read_modify_write_operation_persistent_fence();
		result
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fetch_nand_(&self, increment: bool, ordering: Ordering) -> bool;
}

impl BoolAtomicPersistentMemory for AtomicBool
{
	#[inline(always)]
	fn fetch_nand_(&self, increment: bool, ordering: Ordering) -> bool
	{
		self.fetch_nand(increment, ordering)
	}
}
