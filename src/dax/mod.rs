// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::io;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::io::Read;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::fs::File;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::os::unix::fs::MetadataExt;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::path::Path;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::std::path::PathBuf;


include!("CouldNotObtainDeviceDaxStatisticError.rs");
include!("DaxDevicePathExt.rs");




//static ssize_t
//device_dax_size(const char *path)
//{
//	LOG(3, "path \"%s\"", path);
//
//	os_stat_t st;
//	int olderrno;
//
//	if (os_stat(path, &st) < 0) {
//		ERR("!stat \"%s\"", path);
//		return -1;
//	}
//
//	char spath[PATH_MAX];
//	snprintf(spath, PATH_MAX, "/sys/dev/char/%u:%u/size",
//		os_major(st.st_rdev), os_minor(st.st_rdev));
//
//	LOG(4, "device size path \"%s\"", spath);
//
//	int fd = os_open(spath, O_RDONLY);
//	if (fd < 0) {
//		ERR("!open \"%s\"", spath);
//		return -1;
//	}
//
//	ssize_t size = -1;
//
//	char sizebuf[MAX_SIZE_LENGTH + 1];
//	ssize_t nread;
//	if ((nread = read(fd, sizebuf, MAX_SIZE_LENGTH)) < 0) {
//		ERR("!read");
//		goto out;
//	}
//
//	sizebuf[nread] = 0; /* null termination */
//
//	char *endptr;
//
//	olderrno = errno;
//	errno = 0;
//
//	size = strtoll(sizebuf, &endptr, 0);
//	if (endptr == sizebuf || *endptr != '\n' ||
//	    ((size == LLONG_MAX || size == LLONG_MIN) && errno == ERANGE)) {
//		ERR("invalid device size %s", sizebuf);
//		size = -1;
//		goto out;
//	}
//
//	errno = olderrno;
//
//out:
//	olderrno = errno;
//	(void) os_close(fd);
//	errno = olderrno;
//
//	LOG(4, "device size %zu", size);
//	return size;
//}
//#endif
