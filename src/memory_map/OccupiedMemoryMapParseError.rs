// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


quick_error!
{
	/// Reason for failing to memory map.
	#[derive(Debug)]
	pub enum OccupiedMemoryMapParseError
	{
		/// Input-Output error.
		InputOutput(cause: ::std::io::Error)
		{
			cause(cause)
			description(cause.description())
            display("Could not parse memory maps in /proc because of Input/Output error: {}", cause)
            from()
		}
		
		/// Could not parse a line in the `/proc` map file for this process.
		CouldNotParseProcMap(cause: ::text_io::Error)
		{
			cause(cause)
			description(cause.description())
            display("Could not parse memory maps in /proc: {}", cause)
            from()
		}
		
		/// Could not convert a value in a line in the `/proc` map file for this process.
		CouldNotConvertProcMapValue(cause: ::std::num::ParseIntError)
		{
			cause(cause)
			description(cause.description())
            display("Could not convert value in memory maps in /proc: {}", cause)
            from()
		}
		
		/// Invalid line in the `/proc` map file for this process.
		InvalidLineInProcMap(description: &'static str)
		{
			description(description)
            display("Could not parse line in memory maps in /proc: '{}'", description)
		}
	}
}
