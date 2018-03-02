// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


use super::Alignment;
use super::dax::CouldNotObtainDaxDeviceStatisticError;
use super::dax::DaxDevicePathExt;
#[cfg(unix)] use ::libc::sysconf;
#[cfg(unix)] use ::libc::_SC_PAGESIZE;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::fs::File;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::io::BufRead;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::io::BufReader;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::path::Path;


include!("CouldNotMemoryMapError.rs");
include!("memory_map_page_size.rs");
include!("PersistentMemoryFilePathExt.rs");



#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
#[inline(always)]
fn util_map_hint_unused(minimum_address: *mut u8, length: usize, alignment: usize) -> Result<(), CouldNotMemoryMapError>
{
	debug_assert!(alignment.is_power_of_two(), "alignment must be a power of two");
	
	#[cfg(any(target_os = "android", target_os = "linux"))] const CurrentProcessMapFile: &'static str = "/proc/self/maps";
	//noinspection SpellCheckingInspection
	#[cfg(target_os = "freebsd")] const CurrentProcessMapFile: &'static str = "/proc/curproc/map";
	
	let unaligned_minimum_address = if minimum_address.is_null()
	{
		memory_map_page_size() as *mut u8
	}
	else
	{
		minimum_address
	};
	
	let raddr = unaligned_minimum_address.round_up_to_alignment(alignment);
	
	let mut file = File::open(CurrentProcessMapFile)?;
	
	
	
	
	
	let buffer_reader = BufReader::new(file);
	for line in buffer_reader.lines()
	{
		let line = line?;
		
		// Sample line on Linux: `17add6e9000-17add7ae000 r-xp 00000000 08:03 2228226                      /bin/busybox`
		let low: String;
		let high: String;
		let _remainder: String;
		
		#[cfg(any(target_os = "android", target_os = "linux"))] try_scan!(line.bytes() => "{}-{} {}", low, high, _remainder);
		#[cfg(target_os = "freebsd")] try_scan!(line.bytes() => "{} {} {}", low, high, _remainder);
		
		let low = usize::from_str_radix(&low, 16)?;
		let high = usize::from_str_radix(&high, 16)?;
		
		/*
			
    let a: String = read!("{} ");
    let b: i32 = read!("{} ");
    let c: f32 = read!("{}\n");
		*/
		/*
			#ifdef __FreeBSD__
			static const char *sscanf_os = "%p %p";
			#else
			static const char *sscanf_os = "%p-%p";
			#endif
		*/
	}
	
	Ok(())
}


/*
 * util_map_hint_unused -- use /proc to determine a hint address for mmap()
 *
 * This is a helper function for util_map_hint().
 * It opens up /proc/self/maps and looks for the first unused address
 * in the process address space that is:
 * - greater or equal 'minaddr' argument,
 * - large enough to hold range of given length,
 * - aligned to the specified unit.
 *
 * Asking for aligned address like this will allow the DAX code to use large
 * mappings.  It is not an error if mmap() ignores the hint and chooses
 * different address.
 */
//char *
//util_map_hint_unused(void *minaddr, size_t len, size_t align)
//{
//	LOG(3, "minaddr %p len %zu align %zu", minaddr, len, align);
//	ASSERT(align > 0);
//
//	FILE *fp;
//	if ((fp = os_fopen(Mmap_mapfile, "r")) == NULL) {
//		ERR("!%s", Mmap_mapfile);
//		return MAP_FAILED;
//	}
//
//	char line[PROCMAXLEN];	/* for fgets() */
//	char *lo = NULL;	/* beginning of current range in maps file */
//	char *hi = NULL;	/* end of current range in maps file */
//	char *raddr = minaddr;	/* ignore regions below 'minaddr' */
//
//	if (raddr == NULL)
//		raddr += Pagesize;
//
//	raddr = (char *)roundup((uintptr_t)raddr, align);
//
//	while (fgets(line, PROCMAXLEN, fp) != NULL) {
//		/* check for range line */
//		if (sscanf(line, sscanf_os, &lo, &hi) == 2) {
//			LOG(4, "%p-%p", lo, hi);
//			if (lo > raddr) {
//				if ((uintptr_t)(lo - raddr) >= len) {
//					LOG(4, "unused region of size %zu "
//							"found at %p",
//							lo - raddr, raddr);
//					break;
//				} else {
//					LOG(4, "region is too small: %zu < %zu",
//							lo - raddr, len);
//				}
//			}
//
//			if (hi > raddr) {
//				raddr = (char *)roundup((uintptr_t)hi, align);
//				LOG(4, "nearest aligned addr %p", raddr);
//			}
//
//			if (raddr == NULL) {
//				LOG(4, "end of address space reached");
//				break;
//			}
//		}
//	}
//
//	/*
//	 * Check for a case when this is the last unused range in the address
//	 * space, but is not large enough. (very unlikely)
//	 */
//	if ((raddr != NULL) && (UINTPTR_MAX - (uintptr_t)raddr < len)) {
//		LOG(4, "end of address space reached");
//		raddr = MAP_FAILED;
//	}
//
//	fclose(fp);
//
//	LOG(3, "returning %p", raddr);
//	return raddr;
//}
