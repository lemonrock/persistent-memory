// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


use super::intrinsics::*;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::AtomicPtr;
use ::std::sync::atomic::AtomicI8;
use ::std::sync::atomic::AtomicI16;
use ::std::sync::atomic::AtomicI32;
use ::std::sync::atomic::AtomicI64;
use ::std::sync::atomic::AtomicIsize;
use ::std::sync::atomic::AtomicU8;
use ::std::sync::atomic::AtomicU16;
use ::std::sync::atomic::AtomicU32;
use ::std::sync::atomic::AtomicU64;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::Ordering;
use ::std::sync::atomic::Ordering::Acquire;
use ::std::sync::atomic::Ordering::AcqRel;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::Release;
use ::std::sync::atomic::Ordering::SeqCst;


include!("AtomicPersistentMemory.rs");
include!("IntegerAtomicPersistentMemory.rs");
include!("persistent_fence.rs");
include!("persistent_sync.rs");
include!("persistent_write_back.rs");
