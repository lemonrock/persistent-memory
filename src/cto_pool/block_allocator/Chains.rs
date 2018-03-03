// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Stored in Persistent Memory
pub struct Chains
{
	cto_pool_arc: CtoPoolArc,
	block_allocator: NonNull<BlockAllocator>,
	head_of_chains_linked_list: BlockPointer,
}

impl Drop for Chains
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.head_of_chains_linked_list.is_not_null()
		{
			let block_allocator = self.block_allocator();
			let head = block_allocator.block_meta_data_unchecked(self.head_of_chains_linked_list);
			head.recycle_chains_into_block_allocator(block_allocator, self.head_of_chains_linked_list);
		}
		self.cto_pool_arc.clone().free_pointer(self)
	}
}

impl CtoSafe for Chains
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.block_allocator_mut().cto_pool_opened(cto_pool_arc)
	}
}

impl Chains
{
	#[inline(always)]
	fn new(block_allocator: &BlockAllocator, cto_pool_arc: &CtoPoolArc) -> Result<NonNull<Self>, ()>
	{
		match cto_pool_arc.pool_pointer().aligned_alloc(size_of::<Self>(), size_of::<Self>())
		{
			Err(_) => Err(()),
			Ok(void_pointer) =>
			{
				let mut this = (void_pointer as *mut Self).to_non_null();
				
				this.mutable_reference().initialize(block_allocator.to_non_null(), cto_pool_arc);
				
				Ok(this)
			}
		}
	}
	
	#[inline(always)]
	fn initialize(&mut self, block_allocator: NonNull<BlockAllocator>, cto_pool_arc: &CtoPoolArc)
	{
		unsafe
		{
			write(&mut self.cto_pool_arc, cto_pool_arc.clone());
			write(&mut self.block_allocator, block_allocator.clone());
			write(&mut self.head_of_chains_linked_list, BlockPointer::Null);
		}
	}
	
	#[inline(always)]
	fn block_allocator(&self) -> &BlockAllocator
	{
		self.block_allocator.reference()
	}
	
	#[inline(always)]
	fn block_allocator_mut(&mut self) -> &mut BlockAllocator
	{
		self.block_allocator.mutable_reference()
	}
	
	/// Copy bytes into chains.
	#[inline(always)]
	pub fn copy_bytes_into_chains_start<'block_meta_data>(&'block_meta_data self) -> RestartCopyIntoAt<'block_meta_data>
	{
		let block_allocator = self.block_allocator();
		RestartCopyIntoAt::new(block_allocator.block_size, block_allocator.blocks_memory_inclusive_start_pointer, self.head_of_chains_linked_list, &block_allocator.block_meta_data_items())
	}
	
	/// Copy bytes from chains.
	#[inline(always)]
	pub fn copy_bytes_from_chains_start<'block_meta_data>(&'block_meta_data self) -> RestartCopyFromAt<'block_meta_data>
	{
		let block_allocator = self.block_allocator();
		RestartCopyFromAt::new(block_allocator.block_size, block_allocator.blocks_memory_inclusive_start_pointer, self.head_of_chains_linked_list, &block_allocator.block_meta_data_items())
	}
}
