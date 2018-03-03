// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Stored in Persistent Memory.
/// Uses `#[repr(C)]` to prevent reordering of fields.
/// Variable-sized data is stored after this struct, and so it can not be placed on the stack.
#[repr(C, align(4096))] // `align(Self::Alignment)`.
pub struct BlockAllocator
{
	number_of_blocks: usize,
	
	// Can be computed every time but stored for efficiency.
	block_size: BlockSize,
	blocks_memory_inclusive_start_pointer: NonNull<u8>,
	blocks_memory_exclusive_end_pointer: NonNull<u8>,
	blocks_meta_data_items_inclusive_start_pointer: NonNull<BlockMetaDataItems>,
	
	// A free list.
	bags: Bags,
	
	// We store variable length Blocks at a Self::Alignment byte alignment after the BlockAllocator, ie immediately after the end.
	
	// We store variable length BlockMetaDataItems<B> at a Self::Alignment byte alignment after the Blocks.
}

impl CtoSafe for BlockAllocator
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.bags.cto_pool_opened(cto_pool_arc)
	}
}

impl BlockAllocator
{
	const Alignment: usize = 4096;
	
	/// Calculate how many blocks can be assigned.
	/// Returns 0 (zero) if nothing is possible.
	#[inline(always)]
	pub fn maximum_number_of_blocks(memory_capacity_available_in_bytes: usize, block_size: BlockSize) -> usize
	{
		// We assume a block size for the purposes of calculation that matches Self::Alignment.
		// This means we can calculate without having to worry about alignment rounding-up rules.
		
		let struct_size = Self::offset_to_start_of_variable_length_memory();
		if memory_capacity_available_in_bytes <= struct_size
		{
			return 0;
		}
		
		const _4096: BlockSize = BlockSize::_4096;
		
		let number_of_4096_sized_blocks = (memory_capacity_available_in_bytes - struct_size) / (_4096.as_usize() + size_of::<BlockMetaData>());
		
		let scalar = _4096.as_usize() / block_size.as_usize();
		
		let number_of_blocks = number_of_4096_sized_blocks * scalar;
		
		min(number_of_blocks, BlockPointer::InclusiveMaximumNumberOfBlocks)
	}
	
	/// Size of this object.
	#[inline(always)]
	pub fn size_of(number_of_blocks: usize, block_size: BlockSize) -> usize
	{
		let aligned_struct = Self::offset_to_start_of_variable_length_memory();
		let blocks_capacity = Self::blocks_capacity(number_of_blocks, block_size).round_up_to_alignment(Self::Alignment);
		let meta_data = BlockMetaDataItems::size_of(number_of_blocks);
		aligned_struct + blocks_capacity + meta_data
	}
	
	/// block_size is a minimum of 64 and could be 512 for systems with AVX512 CPU instructions.
	pub fn new(unaligned_address: usize, number_of_blocks: usize, block_size: BlockSize) -> NonNull<Self>
	{
		let aligned_address = unaligned_address.round_up_to_alignment(Self::Alignment);
		
		let mut this = (aligned_address as *mut Self).to_non_null();
		
		this.mutable_reference().initialize(aligned_address, number_of_blocks, block_size);
		
		this
	}
	
	#[inline(always)]
	fn initialize(&mut self, aligned_address: usize, number_of_blocks: usize, block_size: BlockSize)
	{
		assert_ne!(number_of_blocks, 0, "number_of_blocks must not be zero");
		assert!(number_of_blocks < BlockPointer::InclusiveMaximumNumberOfBlocks, "number_of_blocks '{}' can not exceed InclusiveMaximumNumberOfBlocks '{}'", number_of_blocks, BlockPointer::InclusiveMaximumNumberOfBlocks);
		
		let blocks_memory_inclusive_start_pointer = aligned_address + Self::offset_to_start_of_variable_length_memory();
		let blocks_capacity = Self::blocks_capacity(number_of_blocks, block_size);
		let blocks_memory_exclusive_end_pointer = blocks_memory_inclusive_start_pointer + blocks_capacity;
		let blocks_meta_data_items_inclusive_start_pointer = blocks_memory_exclusive_end_pointer.round_up_to_alignment(Self::Alignment);
		
		unsafe
		{
			write(&mut self.number_of_blocks, number_of_blocks);
			write(&mut self.block_size, block_size);
			write(&mut self.blocks_memory_inclusive_start_pointer, (blocks_memory_inclusive_start_pointer as *mut u8).to_non_null());
			write(&mut self.blocks_memory_exclusive_end_pointer, (blocks_memory_exclusive_end_pointer as *mut u8).to_non_null());
			write(&mut self.blocks_meta_data_items_inclusive_start_pointer, (blocks_meta_data_items_inclusive_start_pointer as *mut BlockMetaDataItems).to_non_null());
			write(&mut self.bags, Bags::default());
			
			self.block_meta_data_items_mut().initialize(number_of_blocks);
			
			self.initialize_chains(number_of_blocks);
		}
	}
	
	/// Allocate a chain.
	/// Effectively like `malloc`, but alignment will always be `BlockSize` and `requested_size` will be rounded up to BlockSize.
	/// A request for an allocation larger than `InclusiveMaximumChainLength` (usually 1024 blocks) will result in a null pointer, `BlockPointer::Null`.
	/// For a block size of 64, this means the maximum allocation is 64kb; for a 512 byte block size, it is 512Kb.
	/// A request for a zero-size (empty) allocation, ie `requested_size == 0`, will result in a null pointer, `BlockPointer::Null`.
	/// Second result argument is `chain_length`, ie the number of blocks in the allocation.
	pub fn allocate_chain(&self, requested_size: usize) -> (BlockPointer, usize)
	{
		let number_of_blocks_required = self.block_size.number_of_blocks_required(requested_size);
		
		self.grab_a_chain_exactly_for(number_of_blocks_required)
	}
	
	/// Allocate chains
	pub fn allocate_chains(&self, requested_size: usize, cto_pool_arc: &CtoPoolArc) -> Result<NonNull<Chains>, ()>
	{
		let mut chains = Chains::new(self, cto_pool_arc)?;
		
		let number_of_blocks_required = self.block_size.number_of_blocks_required(requested_size);
		if number_of_blocks_required == 0
		{
			return Ok(chains)
		}
		
		// TODO: Estimate if there is enough memory left before allocating, as it makes failure faster.
		
		let mut number_of_blocks_remaining_to_find = number_of_blocks_required;
		
		let (head_of_chains_linked_list, chain_length) = self.grab_a_chain(number_of_blocks_remaining_to_find);
		if head_of_chains_linked_list.is_null()
		{
			unsafe { drop_in_place(chains.as_ptr()) };
			return Err(())
		}
		chains.mutable_reference().head_of_chains_linked_list = head_of_chains_linked_list;
		
		let mut previous_chain = head_of_chains_linked_list;
		number_of_blocks_remaining_to_find -= chain_length;
		while number_of_blocks_remaining_to_find != 0
		{
			let (next_chain, chain_length) = self.grab_a_chain(number_of_blocks_remaining_to_find);
			let previous_chain_block_meta_data = self.block_meta_data_unchecked(previous_chain);
			if next_chain.is_null()
			{
				// If this isn't done, then who knows what we might free in `drop()`.
				previous_chain_block_meta_data.set_next_chain(BlockPointer::Null);
				unsafe { drop_in_place(chains.as_ptr()) };
				
				return Err(())
			}
			previous_chain_block_meta_data.set_next_chain(next_chain);
			
			previous_chain = next_chain;
			number_of_blocks_remaining_to_find -= chain_length;
		}
		
		self.block_meta_data_unchecked(previous_chain).set_next_chain(BlockPointer::Null);
		
		flush_non_null(chains);
		
		Ok(chains)
	}
	
	#[inline(always)]
	pub(crate) fn to_non_null(&self) -> NonNull<Self>
	{
		(self as *const Self as *mut Self).to_non_null()
	}
	
	#[inline(always)]
	fn offset_to_start_of_variable_length_memory() -> usize
	{
		size_of::<Self>().round_up_to_alignment(Self::Alignment)
	}
	
	#[inline(always)]
	fn blocks_capacity(number_of_blocks: usize, block_size: BlockSize) -> usize
	{
		block_size.total_memory_required_in_bytes(number_of_blocks)
	}
	
	#[inline(always)]
	fn block_meta_data_items(&self) -> &BlockMetaDataItems
	{
		self.blocks_meta_data_items_inclusive_start_pointer.reference()
	}
	
	#[inline(always)]
	fn block_meta_data_items_mut(&mut self) -> &mut BlockMetaDataItems
	{
		self.blocks_meta_data_items_inclusive_start_pointer.mutable_reference()
	}
	
	#[inline(always)]
	fn block_meta_data_unchecked(&self, block_pointer: BlockPointer) -> &BlockMetaData
	{
		block_pointer.expand_to_pointer_to_meta_data_unchecked(self.block_meta_data_items())
	}
	
	fn initialize_chains(&mut self, number_of_blocks: usize)
	{
		let number_of_chains_of_maximum_length = number_of_blocks / InclusiveMaximumChainLength;
		
		let maximum_chain_length = ChainLength::from_length(InclusiveMaximumChainLength);
		
		let mut chain_index = 0;
		while chain_index < number_of_chains_of_maximum_length
		{
			let block_index = chain_index * InclusiveMaximumChainLength;
			let add_block = BlockPointer::new(block_index as u32);
			
			self.bags.add(self.block_meta_data_items(), maximum_chain_length, add_block);
			
			chain_index += 1;
		}
		
		let odd_length_chain = number_of_blocks % InclusiveMaximumChainLength;
		if odd_length_chain != 0
		{
			let block_index = number_of_blocks - odd_length_chain;
			let add_block = BlockPointer::new(block_index as u32);
			
			self.bags.add(self.block_meta_data_items(), ChainLength::from_length(odd_length_chain), add_block);
		}
	}
	
	#[inline(always)]
	pub(crate) fn receive_solitary_chain_back(&self, solitary_chain_block_pointer: BlockPointer)
	{
		debug_assert!(solitary_chain_block_pointer.is_not_null(), "solitary_chain_block_pointer should not be null");
		let solitary_chain_block_meta_data = self.block_meta_data_unchecked(solitary_chain_block_pointer);
		
		// This loop attempts to repeatedly merge more chains onto the end of solitary_chain_block_pointer.
		// Longer chains are better.
		let mut solitary_chain_length = solitary_chain_block_meta_data.chain_length();
		while solitary_chain_length.is_less_than_inclusive_maximum()
		{
			let subsequent_chain_start_address = solitary_chain_block_pointer.subsequent_chain_start_address(self.blocks_memory_inclusive_start_pointer, solitary_chain_length, self.block_size);
			
			if subsequent_chain_start_address.as_ptr() == self.blocks_memory_exclusive_end_pointer.as_ptr()
			{
				break
			}
			
			let cut_chain_block_pointer = BlockPointer::block_address_to_block_pointer(self.blocks_memory_inclusive_start_pointer, subsequent_chain_start_address, self.block_size);
			if self.bags.try_to_cut(self.block_meta_data_items(), cut_chain_block_pointer)
			{
				let cut_chain_block_meta_data = self.block_meta_data_unchecked(cut_chain_block_pointer);
				
				let cut_chain_length = cut_chain_block_meta_data.chain_length();
				match solitary_chain_length.add_if_maximum_length_not_exceeded(cut_chain_length)
				{
					// The newly merged combined chain length may too long.
					// Add the now unwanted cut_chain back to the bags free list.
					None =>
					{
						cut_chain_block_meta_data.reset_before_add_to_bag();
						self.bags.add(self.block_meta_data_items(), cut_chain_length, cut_chain_block_pointer);
						break
					},
					
					Some(combined_chain_length) => solitary_chain_length = combined_chain_length,
				}
				
				solitary_chain_block_meta_data.acquire(solitary_chain_length);
			}
			else
			{
				// Wasn't in the bag, or was stolen by another thread; give up trying to merge chains.
				break
			}
		}
		
		self.nothing_to_merge_with_so_add_to_free_list(solitary_chain_block_pointer, solitary_chain_block_meta_data, solitary_chain_length);
	}
	
	#[inline(always)]
	fn nothing_to_merge_with_so_add_to_free_list(&self, solitary_chain_block_pointer: BlockPointer, solitary_chain_block_meta_data: &BlockMetaData, solitary_chain_length: ChainLength)
	{
		solitary_chain_block_meta_data.reset_before_add_to_bag();
		self.bags.add(self.block_meta_data_items(), solitary_chain_length, solitary_chain_block_pointer)
	}
	
	#[inline(always)]
	fn grab_a_chain(&self, ideal_number_of_blocks: usize) -> (BlockPointer, usize)
	{
		let capped_chain_length = min(ideal_number_of_blocks, InclusiveMaximumChainLength);
		
		// (1) Try to get an exactly right chain or a longer chain.
		//     If the chain is longer, then 'snap off' the right hand side.
		let mut search_for_chain_length = capped_chain_length;
		while search_for_chain_length <= InclusiveMaximumChainLength
		{
			let our_shorter_chain_length = ChainLength::from_length(search_for_chain_length);
			let chain = self.bags.remove(self.block_meta_data_items(), our_shorter_chain_length);
			if chain.is_not_null()
			{
				if search_for_chain_length != capped_chain_length
				{
					chain.expand_to_pointer_to_meta_data_unchecked(self.block_meta_data_items()).snap_off_back_if_longer_than_required_capacity_and_recycle_into_block_allocator(chain, self.blocks_memory_inclusive_start_pointer, our_shorter_chain_length, self);
				}
				return (chain, search_for_chain_length)
			}
			
			search_for_chain_length += 1;
		}
		
		// (2) Try to get a smaller exactly right chain or a smaller chain.
		let mut search_for_chain_length = capped_chain_length;
		while search_for_chain_length > 0
		{
			let chain = self.bags.remove(self.block_meta_data_items(), ChainLength::from_length(search_for_chain_length));
			if chain.is_not_null()
			{
				return (chain, search_for_chain_length)
			}
			
			search_for_chain_length -=1;
		}
		
		(BlockPointer::Null, 0)
	}
	
	#[inline(always)]
	fn grab_a_chain_exactly_for(&self, number_of_blocks: usize) -> (BlockPointer, usize)
	{
		if number_of_blocks == 0 || number_of_blocks > InclusiveMaximumChainLength
		{
			return (BlockPointer::Null, 0)
		}
		
		// Try to get an exactly right chain or a longer chain.
		// If the chain is longer, then 'snap off' the right hand side.
		let mut search_for_chain_length = number_of_blocks;
		while search_for_chain_length <= InclusiveMaximumChainLength
		{
			let our_shorter_chain_length = ChainLength::from_length(search_for_chain_length);
			let chain = self.bags.remove(self.block_meta_data_items(), our_shorter_chain_length);
			if chain.is_not_null()
			{
				if search_for_chain_length != number_of_blocks
				{
					chain.expand_to_pointer_to_meta_data_unchecked(self.block_meta_data_items()).snap_off_back_if_longer_than_required_capacity_and_recycle_into_block_allocator(chain, self.blocks_memory_inclusive_start_pointer, our_shorter_chain_length, self);
				}
				return (chain, search_for_chain_length)
			}
			
			search_for_chain_length += 1;
		}
		
		(BlockPointer::Null, 0)
	}
}
