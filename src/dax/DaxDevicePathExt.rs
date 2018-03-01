// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// An extension trait to provides methods to file paths believed to be DAX devices.
pub trait DaxDevicePathExt
{
	/// Is this file path a device DAX file?
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn is_this_a_dax_device(&self) -> bool;
	
	/// Device DAX alignment.
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn find_dax_device_alignment(&self) -> Result<usize, CouldNotObtainDeviceDaxStatisticError>;
	
	/// Device DAX region id.
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn find_dax_device_region_id(&self) -> Result<usize, CouldNotObtainDeviceDaxStatisticError>;
	
	#[doc(hidden)]
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn find_dax_device_file_statistic_string<Statistic, FileTemplate: FnOnce(u32, u32) -> String, Parser: FnOnce(&str) -> Result<Statistic, CouldNotObtainDeviceDaxStatisticError>>(&self, file_template: FileTemplate, parser: Parser) -> Result<Statistic, CouldNotObtainDeviceDaxStatisticError>;
	
	#[doc(hidden)]
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn character_device_major_and_device_minor(&self) -> Result<(u32, u32), io::Error>;
}

impl DaxDevicePathExt for Path
{
	/// Is this file path a device DAX file?
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn is_this_a_dax_device(&self) -> bool
	{
		match self.character_device_major_and_device_minor()
		{
			Err(_) => false,
			Ok((device_major, device_minor)) =>
				{
					match PathBuf::from(format!("/sys/dev/char/{}:{}/subsystem", device_major, device_minor)).canonicalize()
					{
						Err(_) => false,
						Ok(real_path) => real_path.starts_with("/sys/class/dax")
					}
				}
		}
	}
	
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn find_dax_device_alignment(&self) -> Result<usize, CouldNotObtainDeviceDaxStatisticError>
	{
		self.find_dax_device_file_statistic_string
		(
			|device_major, device_minor| format!("/sys/dev/char/{}:{}/device/align", device_major, device_minor),
			|statistic_string|
			{
				// From Kernel 4.9 onwards the alignment is formatted as a decimal string.
				match statistic_string.parse::<usize>()
				{
					Ok(alignment) => Ok(alignment),
					Err(_) =>
					{
						// Before Kernel 4.9 the alignment is formatted as a hexadecimal string.
						Ok(usize::from_str_radix(statistic_string, 16)?)
					}
				}
			}
		)
	}
	
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn find_dax_device_region_id(&self) -> Result<usize, CouldNotObtainDeviceDaxStatisticError>
	{
		self.find_dax_device_file_statistic_string
		(
			|device_major, device_minor| format!("/sys/dev/char/{}:{}/device/dax_region/id", device_major, device_minor),
			|statistic_string| Ok(statistic_string.parse::<usize>()?)
		)
	}
	
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn find_dax_device_file_statistic_string<Statistic, FileTemplate: FnOnce(u32, u32) -> String, Parser: FnOnce(&str) -> Result<Statistic, CouldNotObtainDeviceDaxStatisticError>>(&self, file_template: FileTemplate, parser: Parser) -> Result<Statistic, CouldNotObtainDeviceDaxStatisticError>
	{
		let device_dax_align_file_path =
		{
			let (device_major, device_minor) = self.character_device_major_and_device_minor()?;
			file_template(device_major, device_minor)
		};
		
		let mut alignment_file = File::open(device_dax_align_file_path)?;
		
		let mut statistic_string = String::with_capacity(64);
		let _bytes_read = alignment_file.read_to_string(&mut statistic_string)?;
		
		if !statistic_string.ends_with('\n')
		{
			return Err(CouldNotObtainDeviceDaxStatisticError::StringDidNotEndWithLineFeed)
		}
		let statistic_string = &statistic_string[.. statistic_string.len() - 1];
		
		parser(statistic_string)
	}
	
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
	#[inline(always)]
	fn character_device_major_and_device_minor(&self) -> Result<(u32, u32), io::Error>
	{
		#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::major;
		#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::minor;
		
		#[cfg(target_os = "freebsd")] use ::libc::dev_t;
		
		// Not unsafe in the slightest but made `unsafe` to match the libc API for Linux.
		#[cfg(target_os = "freebsd")]
		unsafe fn major(dev: dev_t) -> u32
		{
			(dev >> 32) as i32 as u32
		}
		
		// Not unsafe in the slightest but made `unsafe` to match the libc API for Linux.
		#[cfg(target_os = "freebsd")]
		unsafe fn minor(dev: dev_t) -> u32
		{
			dev as i32 as u32
		}
		
		let character_device = self.metadata()?.rdev();
		
		Ok((unsafe { major(character_device) }, unsafe { minor(character_device) }))
	}
}
