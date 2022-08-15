// SPDX-License-Identifier: (GPL-2.0-or-later OR MIT)
//
// Copyright (c) 2022 SUSE LLC
//
// Author: Joerg Roedel <jroedel@suse.de>
//
// vim: ts=4 sw=4 et

use core::arch::asm;

pub const EFER      : u32 = 0xC000_0080;
pub const SEV_STATUS    : u32 = 0xC001_0131;
pub const SEV_GHCB  : u32 = 0xC001_0130;
pub const MSR_GS_BASE   : u32 = 0xC000_0101;

pub fn read_msr(msr : u32) -> u64 {
    let eax : u32;
    let edx : u32;

    unsafe {
        asm!("rdmsr",
             in("ecx") msr,
             out("eax") eax,
             out("edx") edx,
             options(att_syntax));
    }
    (eax as u64) | (edx as u64) << 32
}

pub fn write_msr(msr: u32, val: u64) {
    let eax = (val & 0x0000_0000_ffff_ffff) as u32;
    let edx = (val >> 32) as u32;

    unsafe {
        asm!("wrmsr",
             in("ecx") msr,
             in("eax") eax,
             in("edx") edx,
             options(att_syntax));
    }
}
