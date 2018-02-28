// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[cfg(not(all(target_feature = "rdrnd", any(target_arch = "x86", target_arch = "x86_64"))))] use ::rand::Rng;
#[cfg(not(all(target_feature = "rdrnd", any(target_arch = "x86", target_arch = "x86_64"))))] use ::rand::thread_rng;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::Ordering::Relaxed;


include!("current_hyper_thread_index.rs");
include!("generate_hyper_thread_safe_random_usize.rs");
include!("hyper_thread_index.rs");
include!("maximum_number_of_hyper_threads.rs");
include!("MaximumSupportedHyperThreads.rs");
