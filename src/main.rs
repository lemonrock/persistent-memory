// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


#![feature(static_nobundle)]


extern crate libc;
extern crate persistent_memory;


fn main()
{
	println!("hello world '{:?}'", unsafe { pool_create(50) });
}


use ::libc::c_void;
use ::libc::size_t;

#[link(name = "jemalloc_pic", kind = "static-nobundle")]
extern "C"
{
	#[link_name = "_persistent_memory_je_pool_create"]
	pub fn pool_create(size: size_t) -> *mut c_void;
}

/*
	Allocator	Possibility											Notes
	
	None		PROBABLY											Use FreeList with a fixed set of allocation sizes
																	Rules out using CtoVec, CtoString
																	Complex for the Elimination Array in block_allocator, but can be solved
																	Known memory constraints allow us to arguably handle low memory situations better
																	BlockAllocator is a client of free list, perhaps.
																	
	tcmalloc	NO													No way to specify a piece of memory to use
																	C++
																	
	jemalloc 5	Maybe if can access MIB functionality directly		Need to use the `mallctl` MIB API with `arena.<i>.chunk_hooks` as can assign an arena per thread so constantly switching arenas before alloc / free.
																	https://stackoverflow.com/questions/16933103/using-tcmalloc-jemalloc-with-custom-memory-pool-manager?noredirect=1&lq=1
																	http://jemalloc.net/jemalloc.3.html#arenas.create
																	Bug in compiling with custom prefix (needed to avoid clash with Rust)
																	
	jemalloc 4	Maybe if can access MIB functionality directly		Similar issues to jemalloc 5, but might compile without the custom prefix bug (untested)
	
	PMDK jemalloc 3.6	YES											Customized and 4 years old codebase; customizations of unknown provenance
																	A lot of C code to import from a project we no longer trust not to break stuff, although it is unlikely that this particular code will change often
	
	Hoard		UNKNOWN												GPL with expensive license fees ($20,000 for redistribution rights)
																	C++
																	
	Remap Heap	NO													https://stackoverflow.com/questions/23341587/malloc-like-function-using-custom-heap, answer 2\*
	
	libpmemobj	MAYBE												Uses relative pointers; may be we could do 'on open' pointer re-basing? [especially if we store previous base somewhere]
																	A lot of C code to import from a project we no longer trust not to break stuff
	
	nvm-malloc	MAYBE												3-year old code
																	Designed for obsolete PMFS
																	Uses relative pointers, so need a relative => absolute pointer mapping
																	https://github.com/IMCG/nvm-malloc
	
	PtAllocator	UNKNOWN												http://www.vldb.org/pvldb/vol10/p1166-oukid.pdf
	
	SuperMalloc	UNKNOWN												Uses Intel TSX, so looks interesting
	
	ElfMalloc	UNKNOWN
	
	Atlas/makalu	YES												https://github.com/HewlettPackard/Atlas/tree/makalu
																	Simple, region-based alloc() / free(); no alignment (we'd have to this ourselves)
																	Root pointer
																	Garbage collection?
																	Recovery
																	BUT... doesn't seem to work with DAX?
																	BUT... restricted to clflush
																	C++
																	Code in pmalloc.cpp looks quite interesting
																	
	
	'nvram'		PROBABLY NOT										https://github.com/efficient/nvram/
																	Looks Dead; 5-year old code
	
	
	
	
	
	Other
		Embed StrongArc<X> directly into FAA Queue
			Works as we can actually insert any 8-byte value into the queue as long as it is not 0 or !0.
		SSE2 and AVX2 optimized memcpy / memset routines
	
	\*
		A relatively easy way to use the standard memory allocator malloc is to remap the default heap with your custom mapping.

		void * part_of_heap = memalign(sysconf(_SC_PAGESIZE), nbytes);
		void * ret = mmap(part_of_heap, nbytes
					 , PROT_READ | PROT_WRITE, MAP_FIXED, fd, 0);
		if (ret == MAP_FAILED) {/* ... */}
		free(part_of_heap);
		
		Now anything that is placed in the area part_of_heap-part_of_heap+nbytes by malloc will go into your own mapped area. This is unsupported though and will not guarantee that any allocations will actually go there.
	
*/
