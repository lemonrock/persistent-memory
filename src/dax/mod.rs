// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::major;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::minor;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::fs::File;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::path::PathBuf;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::path::Path;

quick_error!
{
	/// Reason for failing to obtain Device DAX statistic.
	#[derive(Debug)]
	pub enum CouldNotObtainDeviceDaxStatisticError
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

/// Device DAX alignment.
/// An implementation for FreeBSD is possible, but will need to supply `libc::major` and `libc::minor` and access an alternative to `st_rdev()`.
#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline(always)]
pub fn device_dax_alignment(device_dax_path: &Path) -> Result<(), CouldNotObtainDeviceDaxStatisticError>
{
	let device_dax_align_file_path =
	{
		let character_device = path.metadata()?.st_rdev();
		format!("/sys/dev/char/{}:{}/device/align", major(character_device), minor(character_device));
	};
	
	let mut alignment_file = File::open(device_dax_align_file_path)?;
	
	let mut statistic_string = String::with_capacity(64);
	let _bytes_read = alignment_file.read_to_string(&mut statistic_string)?;
	
	if !statistic_string.ends_with('\n')
	{
		return Err(CouldNotObtainDeviceDaxStatisticError::StringDidNotEndWithLineFeed)
	}
	let statistic_string = &[.. statistic_string.len() - 1];
	
	// From Kernel 4.9 onwards the alignment is formatted as a decimal string.
	match statistic_string::parse::<usize>()
	{
		Ok(alignment) => Ok(alignment),
		Err(_) =>
		{
			// Before Kernel 4.9 the alignment is formatted as a hexadecimal string.
			let alignment = usize::from_str_radix(statistic_string, 16)?;
			Ok(alignment)
		}
	}
}

/// Device DAX region id.
/// An implementation for FreeBSD is possible, but will need to supply `libc::major` and `libc::minor` and access an alternative to `st_rdev()`.
#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline(always)]
pub fn find_device_dax_region_id(device_dax_path: &Path) -> Result<i32, CouldNotObtainDeviceDaxStatisticError>
{
	let device_dax_region_id_file_path =
	{
		let character_device = path.metadata()?.st_rdev();
		format!("/sys/dev/char/{}:{}/device/dax_region/id", major(character_device), minor(character_device));
	};
	
	let mut region_id_file = File::open(device_dax_align_file_path)?;
	
	if !statistic_string.ends_with('\n')
	{
		return Err(CouldNotObtainDeviceDaxStatisticError::StringDidNotEndWithLineFeed)
	}
	let statistic_string = &[.. statistic_string.len() - 1];
	
	let statistic = statistic_string::parse::<usize>()?;
	Ok(statistic)
}
