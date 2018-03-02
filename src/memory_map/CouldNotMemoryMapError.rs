// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


quick_error!
{
	/// Reason for failing to memory map.
	#[derive(Debug)]
	pub enum CouldNotMemoryMapError
	{
		/// Input-Output error.
		InputOutput(cause: ::std::io::Error)
		{
			cause(cause)
			description(cause.description())
            display("Could not memory map because of Input/Output error: {}", cause)
            from()
		}
		
		/// Could not obtain a DAX device statistic.
		CouldNotObtainDaxDeviceStatistic(cause: CouldNotObtainDaxDeviceStatisticError)
		{
			cause(cause)
			description(cause.description())
            display("Could not memory map because of DAX device error: {}", cause)
            from()
		}
		
		/// Could not find a contiguous region to memory into
		CouldNotFindAContiguousRegionToMemoryMapInto(size: u64, alignment: usize)
		{
			description("could not find a contiguous region to memory map into")
            display("Could not memory map because could not find a contiguous region to memory map into for size '{}' and alignment '{}'", size, alignment)
		}
		
		/// Could not parse a line in the `/proc` map file for this process.
		CouldNotParseProcMap(cause: ::text_io::Error)
		{
			cause(cause)
			description(cause.description())
            display("Could not memory map because could not parse proc map: {}", cause)
            from()
		}
		
		/// Could not convert a value in a line in the `/proc` map file for this process.
		CouldNotConvertProcMapValue(cause: ::std::num::ParseIntError)
		{
			cause(cause)
			description(cause.description())
            display("Could not memory map because could not convert value from proc map: {}", cause)
            from()
		}
		
		/// The `mmap` libc called failed.
		MMapFailed
		{
			description("mmap failed")
			display("mmap failed")
		}
	}
}
