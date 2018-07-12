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
#![no_std]
#![feature(lang_items)]
#![feature(extern_prelude)]

#[no_mangle]
#[lang = "panic_impl"]
pub fn panic_impl(pi: &core::panic::PanicInfo) -> ! { loop {} }

mod log_writer;

use log_writer::Writer;
use log_writer::VgaColor;


#[no_mangle]
pub fn kernel_main() 
{
	let mut writer = Writer::new();

	writer.print_ln_std("Hello world!");
	writer.print_ln_info("Hello info!");
	writer.print_ln_debug("Hello debug!");
	writer.print_ln_warning("Hello warning!");
	writer.print_ln_error("Hello error!");
	writer.print_ln_custom_color("Hello custom!", VgaColor::Magenta);
}
