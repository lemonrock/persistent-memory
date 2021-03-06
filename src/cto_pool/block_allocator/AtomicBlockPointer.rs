// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct AtomicBlockPointer(AtomicU32);

impl Default for AtomicBlockPointer
{
	#[inline(always)]
	fn default() -> Self
	{
		let block_pointer = BlockPointer::default();
		AtomicBlockPointer(AtomicU32::new(block_pointer.0))
	}
}

impl AtomicBlockPointer
{
	#[inline(always)]
	fn get_relaxed(&self) -> BlockPointer
	{
		BlockPointer::new(self.0.load(Relaxed))
	}
	
	#[inline(always)]
	fn set_relaxed(&self, new_block_pointer: BlockPointer)
	{
		self.0.store(new_block_pointer.0, Relaxed)
	}
	
	#[inline(always)]
	fn get(&self) -> BlockPointer
	{
		BlockPointer::new(self.0.load(Acquire))
	}
	
	#[inline(always)]
	fn set(&self, new_block_pointer: BlockPointer)
	{
		self.0.store(new_block_pointer.0, Release)
	}
}
