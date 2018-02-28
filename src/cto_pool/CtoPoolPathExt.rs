// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// An extension trait to make it easier to use a Path to access a CTO pool.
pub trait CtoPoolPathExt
{
	/// Validate that an existing CTO pool is consistent.
	#[inline(always)]
	fn validate_cto_pool_is_consistent(&self, layout_name: &CStr) -> Result<bool, PmdkError>;
	
	/// Open an existing CTO pool.
	#[inline(always)]
	fn open_cto_pool(&self, layout_name: &CStr) -> Result<*mut PMEMctopool, PmdkError>;
	
	/// Create (and implicitly open) a new CTO pool.
	#[inline(always)]
	fn create_cto_pool(&self, layout_name: &CStr, pool_size: usize, mode: mode_t) -> Result<*mut PMEMctopool, PmdkError>;
}

impl CtoPoolPathExt for Path
{
	#[inline(always)]
	fn validate_cto_pool_is_consistent(&self, _layout_name: &CStr) -> Result<bool, PmdkError>
	{
		unimplemented!()
	}
	
	#[inline(always)]
	fn open_cto_pool(&self, _layout_name: &CStr) -> Result<*mut PMEMctopool, PmdkError>
	{
		unimplemented!()
	}
	
	#[inline(always)]
	fn create_cto_pool(&self, _layout_name: &CStr, _pool_size: usize, _mode: mode_t) -> Result<*mut PMEMctopool, PmdkError>
	{
		unimplemented!()
	}
}
