// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


quick_error!
{
	/// Reason for failing to obtain Device DAX statistic.
	#[derive(Debug)]
	pub enum CouldNotObtainDaxDeviceStatisticError
	{
		/// Input-Output error.
		InputOutput(cause: ::std::io::Error)
		{
			cause(cause)
			description(cause.description())
            display("Could not read device DAX alignment because of Input/Output error: {}", cause)
            from()
		}
		
		/// Alignment string did not end with line-feed
		StringDidNotEndWithLineFeed
		{
			description("string did not end with line-feed")
			display("Device DAX statistic string did not end with line-feed")
		}
		
		/// Alignment string could not be parsed
		StringCouldNotBeParsed(cause: ::std::num::ParseIntError)
		{
			cause(cause)
			description(cause.description())
            display("Could not parse device DAX statistic because of string formatting error: {}", cause)
            from()
		}
	}
}
