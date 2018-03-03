// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


use super::Alignment;
use super::ToNonNull;
use super::dax::CouldNotObtainDaxDeviceStatisticError;
use super::dax::DaxDevicePathExt;
#[cfg(unix)] use ::libc::_SC_PAGESIZE;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::c_int;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::c_void;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::MAP_ANONYMOUS;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::MAP_FAILED;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::MAP_FIXED;
#[cfg(target_os = "freebsd")] use ::libc::MAP_EXCL;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::MAP_PRIVATE;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::MAP_SHARED;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::mmap;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::munmap;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::PROT_READ;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::PROT_WRITE;
#[cfg(unix)] use ::libc::sysconf;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::collections::BTreeMap;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::fs::File;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::fs::OpenOptions;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::io::BufRead;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::io::BufReader;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::os::unix::io::AsRawFd;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::path::Path;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::ptr::NonNull;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::ptr::null_mut;


include!("CouldNotMemoryMapError.rs");
include!("find_lowest_unoccupied_address_in_process_map.rs");
include!("find_random_memory_map_unoccupied_address.rs");
include!("MappedMemory.rs");
include!("memory_map_page_size.rs");
include!("MemoryMapAddress.rs");
include!("MMapFlags.rs");
include!("OccupiedMemoryMapParseError.rs");
include!("PersistentMemoryFilePathExt.rs");
