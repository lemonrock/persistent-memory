// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Represents a block size in bytes.
/// The default is 256 bytes, which is a sensible default for modern systems with AVX2.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(usize)]
pub enum BlockSize
{
	/// Choose this for allocating small blocks that align to the cache.
	_64 = 64,
	
	/// Choose this for allocating small blocks that double-align to the cache to avoid false sharing.
	_128 = 128,
	
	/// Choose this for systems with AVX2 instructions.
	_256 = 256,
	
	/// Choose this for systems with AVX-512 instructions.
	_512 = 512,

	/// Do not use this ordinarily.
	_4096 = 4096,
}

impl Default for BlockSize
{
	#[inline(always)]
	fn default() -> Self
	{
		BlockSize::_256
	}
}

impl BlockSize
{
	#[inline(always)]
	fn for_alignment(alignment: usize) -> Self
	{
		use self::BlockSize::*;
		
		match alignment
		{
			64 => _64,
			128 => _128,
			256 => _256,
			512 => _512,
			4096 => _4096,
			_ => panic!("Unrecognised alignment to convert to BlockSize '{}'", alignment)
		}
	}
	
	#[inline(always)]
	fn total_memory_required_in_bytes(self, number_of_blocks: usize) -> usize
	{
		self.as_usize() * number_of_blocks
	}
	
	#[inline(always)]
	fn relative_memory_address(self, block_index: u32) -> usize
	{
		self.as_usize() * (block_index as usize)
	}
	
	#[inline(always)]
	fn size_of_chain_in_bytes(self, length_of_chain: usize) -> usize
	{
		self.as_usize() * length_of_chain
	}
	
	#[inline(always)]
	fn offset_into_block_is_zero(self, relative_memory_address: usize) -> bool
	{
		(relative_memory_address % self.as_usize()) == 0
	}
	
	#[inline(always)]
	fn block_index(self, relative_memory_address: usize) -> u32
	{
		(relative_memory_address / self.as_usize()) as u32
	}
	
	#[inline(always)]
	fn number_of_blocks_required(self, requested_size: usize) -> usize
	{
		let block_size_in_bytes = self.as_usize();
		
		let remainder = requested_size % block_size_in_bytes;
		
		if remainder == 0
		{
			requested_size / block_size_in_bytes
		}
		else
		{
			(requested_size / block_size_in_bytes) + 1
		}
	}
	
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self as usize
	}
}
