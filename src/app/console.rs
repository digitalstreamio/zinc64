/*
 * Copyright (c) 2016-2017 Sebastian Jastrzebski. All rights reserved.
 *
 * This file is part of zinc64.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use zinc64::system::C64;

pub struct ConsoleApp {
    c64: C64,
}

impl ConsoleApp {
    pub fn new(c64: C64) -> ConsoleApp {
        ConsoleApp {
            c64
        }
    }

    pub fn run(&mut self) {
        let mut overflow_cycles = 0i32;
        loop {
            overflow_cycles = self.c64.run_frame(overflow_cycles);
            if self.c64.is_cpu_jam() {
                let cpu = self.c64.get_cpu();
                warn!(target: "main", "CPU JAM detected at 0x{:x}", cpu.borrow().get_pc());
                break;
            }
        }
    }
}