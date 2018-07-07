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

pub enum VgaColor 
{
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    Gray,
    DarkGray,
    BrigthBlue,
    BrigthGreen,
    BrigthCyan,
    BrigthRed,
    BrigthMagenta,
    Yellow,
    White,
}

// les pointeurs vers la sortie VGA doivent être u32 cars nous utilisons un système 32bit !
pub struct Writer
{
	video_output: u32,
	character_index: u32
}

impl Writer
{
	fn vga_color_to_u8(&self, color: &VgaColor) -> u16
	{
		match *color 
		{
		    VgaColor::Black => 0x000 ,
		    VgaColor::Blue => 0x100 ,
		    VgaColor::Green => 0x200 ,
		    VgaColor::Cyan => 0x300 ,
		    VgaColor::Red => 0x400 ,
		    VgaColor::Magenta => 0x500 ,
		    VgaColor::Brown => 0x600 ,
		    VgaColor::Gray => 0x700 ,
		    VgaColor::DarkGray => 0x800 ,
		    VgaColor::BrigthBlue => 0x900 ,
		    VgaColor::BrigthGreen => 0xA00 ,
		    VgaColor::BrigthCyan => 0xB00 ,
		    VgaColor::BrigthRed => 0xC00 ,
		    VgaColor::BrigthMagenta => 0xD00 ,
		    VgaColor::Yellow => 0xE00 ,
		    _ => 0xF00 ,
		}
	}

	fn clear_screen(&mut self)
	{
		self.character_index = 0;

		for character_index in 0..(80 * 25)
		{
			let mut character_to_print =  0xF00 | (' ' as u16);
			unsafe { *((self.video_output + self.character_index * 2) as *mut u16) = character_to_print; }

			self.character_index += 1;
		}

		self.character_index = 0;
	}

	fn print_to_screen(&mut self, text: &str, color: VgaColor, new_line: bool)
	{
		// On vérifi si on est sur la dernière ligne
		if self.character_index > ((80 * 24) + 1)
			{ self.clear_screen(); }

		for character in text.bytes()
		{
			let mut character_to_print =  self.vga_color_to_u8(&color) | (character as u16);

			unsafe { *((self.video_output + self.character_index * 2) as *mut u16) = character_to_print; }

			self.character_index += 1;
		}

		// fin de l'impression à l'écran

		if new_line
		{
			let index_to_go = (79 - (self.character_index % 80)) + self.character_index;

			while self.character_index <= index_to_go
			{
				let mut character_to_print =  self.vga_color_to_u8(&color)  | (' ' as u16);
				unsafe { *((self.video_output + self.character_index * 2) as *mut u16) = character_to_print; }

				self.character_index += 1;
			}
		}
		
	}

	pub fn print_std(&mut self, text: &str){ self.print_to_screen(text, VgaColor::White, false); }
	pub fn print_info(&mut self, text: &str){ self.print_to_screen(text, VgaColor::Blue, false); }
	pub fn print_debug(&mut self, text: &str){ self.print_to_screen(text, VgaColor::Green, false); }
	pub fn print_warning(&mut self, text: &str){ self.print_to_screen(text, VgaColor::Yellow, false); }
	pub fn print_error(&mut self, text: &str){ self.print_to_screen(text, VgaColor::Red, false); }
	pub fn print_custom_color(&mut self, text: &str, color: VgaColor){ self.print_to_screen(text, color, false); }

	pub fn print_ln_std(&mut self, text: &str){ self.print_to_screen(text, VgaColor::White, true); }
	pub fn print_ln_info(&mut self, text: &str){ self.print_to_screen(text, VgaColor::Blue, true); }
	pub fn print_ln_debug(&mut self, text: &str){ self.print_to_screen(text, VgaColor::Green, true); }
	pub fn print_ln_warning(&mut self, text: &str){ self.print_to_screen(text, VgaColor::Yellow, true); }
	pub fn print_ln_error(&mut self, text: &str){ self.print_to_screen(text, VgaColor::Red, true); }
	pub fn print_ln_custom_color(&mut self, text: &str, color: VgaColor){ self.print_to_screen(text, color, true); }

	pub fn new() -> Self
	{
		Writer
		{
			video_output: 0xb8000,
			character_index: 0,
		}
	}
}
