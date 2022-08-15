// SPDX-License-Identifier: (GPL-2.0-or-later OR MIT)
//
// Copyright (c) 2022 SUSE LLC
//
// Author: Joerg Roedel <jroedel@suse.de>
//
// vim: ts=4 sw=4 et

pub mod alloc;
pub mod vec;

pub use alloc::{alloc, dealloc, alloc_zeroed, realloc, handle_alloc_error};