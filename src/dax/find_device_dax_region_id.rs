// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


/// Device DAX region id.
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
#[inline(always)]
pub fn find_device_dax_region_id(device_dax_path: &Path) -> Result<usize, CouldNotObtainDeviceDaxStatisticError>
{
	device_dax_file_statistic_string
	(
		device_dax_path,
		|device_major, device_minor| format!("/sys/dev/char/{}:{}/device/dax_region/id", device_major, device_minor),
		|statistic_string| Ok(statistic_string.parse::<usize>()?)
	)
}
