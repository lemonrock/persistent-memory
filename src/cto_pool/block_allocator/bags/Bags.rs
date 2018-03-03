// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


pub(crate) struct Bags
{
	bags: [Bag; InclusiveMaximumChainLength],
}

impl Default for Bags
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			bags:
			{
				let mut array: [Bag; InclusiveMaximumChainLength] = unsafe { uninitialized() };
				
				for bag in array.iter_mut()
				{
					unsafe { write(bag, Bag::default()) }
				}
				
				array
			},
		}
	}
}

impl CtoSafe for Bags
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		for bag in self.bags.iter_mut()
		{
			bag.cto_pool_opened(cto_pool_arc)
		}
	}
}

impl Bags
{
	#[inline(always)]
	pub(crate) fn add(&self, block_meta_data_items: &BlockMetaDataItems, chain_length: ChainLength, add_block: BlockPointer)
	{
		debug_assert!(add_block.is_not_null(), "add_block should not be null");
		
		let bag = chain_length.get_bag(&self.bags);
		bag.add(chain_length, add_block, block_meta_data_items)
	}
	
	#[inline(always)]
	pub(crate) fn remove(&self, block_meta_data_items: &BlockMetaDataItems, chain_length: ChainLength) -> BlockPointer
	{
		let bag = chain_length.get_bag(&self.bags);
		bag.remove(chain_length, block_meta_data_items)
	}
	
	#[inline(always)]
	pub(crate) fn try_to_cut(&self, block_meta_data_items: &BlockMetaDataItems, might_not_be_in_bag_block: BlockPointer) -> bool
	{
		debug_assert!(might_not_be_in_bag_block.is_not_null(), "might_not_be_in_bag_block should not be null");
		
		let might_not_be_in_bag_block_meta_data = might_not_be_in_bag_block.expand_to_pointer_to_meta_data_unchecked(block_meta_data_items);
		
		let mut chain_length_and_bag_stripe_index = might_not_be_in_bag_block_meta_data.chain_length_and_bag_stripe_index();
		while let Some(bag_stripe_index) = chain_length_and_bag_stripe_index.bag_stripe_index()
		{
			let chain_length = chain_length_and_bag_stripe_index.chain_length();
			let bag = chain_length.get_bag(&self.bags);
			
			if bag.try_to_cut(chain_length, might_not_be_in_bag_block, might_not_be_in_bag_block_meta_data, block_meta_data_items, bag_stripe_index)
			{
				return true
			}
			
			spin_loop_hint();
			chain_length_and_bag_stripe_index = might_not_be_in_bag_block_meta_data.chain_length_and_bag_stripe_index();
		}
		
		false
	}
}
