// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * libxas: Extendable Assembler Library
 * Copyright (C) 2022 Amy Parker <apark0006@student.cerritos.edu>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA or visit
 * the GNU Project at <https://gnu.org/licenses>. The GNU General Public
 * License version 2 is available at, for your convenience,
 * <https://gnu.org/licenses/old-licenses/gpl-2.0.html>.
 */

#[derive(Clone, Debug)]
pub enum PlatformArch {
    ChipEightRaw,
}

#[derive(Clone, Debug)]
pub enum PlatformTarget {
    RawBinary,
}

#[derive(Clone, Debug)]
pub struct Platform {
    pub arch: PlatformArch,
    pub target: PlatformTarget,
}

// TODO: proper error handling, return Result<Err> instead of panic
// TODO: overall since this is a library everything needs to be converted
// to error types, the library should NEVER panic - that's the responsibility
// of executables!
// TODO: general consideration CROSS-LIBRARY, change String to &String wherever
// possible. Allows passing string literals (&String = &str) and reduces call
// changes between str and String.
impl Platform {
    pub fn from_platform_info(arch: String, target: String) -> Self {
        Platform {
            // TODO: consider using some kind of lookup table?
            arch: match arch.to_lowercase().as_str() {
                "chipeightraw" | "chip8-raw" | "c8r" | "chip8r" => PlatformArch::ChipEightRaw,
                _ => panic!("unsupported arch"),
            },
            target: match target.to_lowercase().as_str() {
                "bin" | "binary" | "raw" | "rawbin" | "rawbinary" => PlatformTarget::RawBinary,
                _ => panic!("unsupported target"),
            },
        }
    }
}
