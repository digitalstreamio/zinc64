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

use std::cell::Cell;

pub struct Clock {
    counter: Cell<u64>,
}

impl Clock {
    pub fn new() -> Clock {
        Clock { counter: Cell::new(0) }
    }

    #[inline]
    pub fn get(&self) -> u64 {
        self.counter.get()
    }

    #[inline]
    pub fn tick(&self) {
        let result = self.counter.get().wrapping_add(1);
        self.counter.set(result);
    }
}