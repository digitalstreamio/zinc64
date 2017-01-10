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

#[derive(Copy, Clone)]
pub struct Dimension {
    pub width: u16,
    pub height: u16,
}

impl Dimension {
    pub fn new(width: u16, height: u16) -> Dimension {
        Dimension {
            width: width,
            height: height,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}

impl Rect {
    pub fn new(left: u16, right: u16, top: u16, bottom: u16) -> Rect {
        Rect {
            left: left,
            right: right,
            top: top,
            bottom: bottom,
        }
    }

    pub fn new_with_dim(left: u16, top: u16, size: Dimension) -> Rect {
        Rect {
            left: left,
            right: left + size.width - 1,
            top: top,
            bottom: top + size.height - 1,
        }
    }

    pub fn size(&self) -> Dimension {
        Dimension::new(self.right - self.left + 1, self.bottom - self.top + 1)
    }
}
