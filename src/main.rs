// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


extern crate libc;
extern crate jemalloc_sys;

fn main()
{
	println!("hello world '{:?}'", unsafe { pool_create(50) });
}

use ::libc::c_void;
use ::libc::size_t;

extern "C"
{
	#[link_name = "_rjem_pool_create"]
	pub fn pool_create(size: size_t) -> *mut c_void;
}



// Optimized memory copy routines for x86_64, up to AVX2
// jemalloc
