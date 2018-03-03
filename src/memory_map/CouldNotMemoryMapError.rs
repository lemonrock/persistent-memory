// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


quick_error!
{
	/// Could not memory map error.
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
		
		/// Could not parse the occupied memory map.
		CouldNotParseOccupiedMemoryMap(cause: OccupiedMemoryMapParseError)
		{
			cause(cause)
			description(cause.description())
			display("Could not memory map because of occupied memory map parse error: {}", cause)
			from()
		}
		
		/// Could not find a contiguous region to memory into
		CouldNotFindAContiguousRegionToMemoryMapInto(size: u64, alignment: usize)
		{
			description("could not find a contiguous region to memory map into")
			display("Could not memory map because could not find a contiguous region to memory map into for size '{}' and alignment '{}'", size, alignment)
		}
		
		/// Could not obtain a DAX device statistic.
		CouldNotObtainDaxDeviceStatistic(cause: CouldNotObtainDaxDeviceStatisticError)
		{
			cause(cause)
			description(cause.description())
			display("Could not memory map because of DAX device error: {}", cause)
			from()
		}
		
		/// The `mmap` libc call failed.
		MMapFailed
		{
			description("mmap failed")
			display("mmap failed")
		}
	}
}
