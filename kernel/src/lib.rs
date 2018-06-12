/* 
Copyright 2018 rOSty team
This file is part of rOSty.
rOSty is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rOSty is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
You should have received a copy of the GNU General Public License
along with rOSty.  If not, see <http://www.gnu.org/licenses/>.
*/

#![allow(unused)]

// https://doc.rust-lang.org/nightly/core/
#![no_std] 
#![feature(lang_items)]

use core::panic::PanicInfo;

#[lang = "eh_personality"]
fn eh_personality() {}

#[lang = "panic_impl"]
fn panic_impl(pi: &PanicInfo) -> ! { loop {} }

#[no_mangle]
pub fn kernel_main() {
	// TODO
	loop {}
}
