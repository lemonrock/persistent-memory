// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::io;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::io::Read;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::fs::File;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::os::unix::fs::MetadataExt;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::path::Path;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::path::PathBuf;


include!("CouldNotObtainDaxDeviceStatisticError.rs");
include!("DaxDevicePathExt.rs");
include!("PersistentMemoryFilePathExt.rs");
