// This file is part of zinc64.
// Copyright (c) 2016-2019 Sebastian Jastrzebski. All rights reserved.
// Licensed under the GPLv3. See LICENSE file in the project root for full license text.

pub mod board;
pub mod frame_buffer;
pub mod gpio;
pub mod mbox;
pub mod mmu;
pub mod uart;

const MMIO_BASE: u32 = 0x3F00_0000;