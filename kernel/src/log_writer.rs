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

type SystemArch = u32;
type CharSize = u16;

const MAX_CHARS: u32 = 80;
const MAX_LINES: u32 = 3;
const UMAX_CHARS: usize = MAX_CHARS as usize;
const UMAX_LINES: usize = MAX_LINES as usize;

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

pub struct Writer
{			
	video_output: SystemArch,							
	character_index: SystemArch,						
	buffer: [ [ CharSize; UMAX_CHARS ] ; UMAX_LINES ],
	buffer_line_index: SystemArch,		
	buffer_char_index: SystemArch,
	start_print_index: SystemArch	
}

impl Writer
{
	fn vga_color_to_u8(&self, color: &VgaColor) -> CharSize
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

	fn clear_line_buffer(&mut self, line_index: usize)
	{
		for char_index in 0..MAX_CHARS
		{
			self.buffer[ line_index ][ (char_index as usize) ] = (0xF00 | (' ' as CharSize)) ;
		}
	}

	fn clear_buffer(&mut self)
	{
		for line in 0..MAX_LINES
		{
			self.clear_line_buffer(line as usize);
		}

		self.buffer_line_index = 0;
		self.buffer_char_index = 0;
	}

	fn clear_screen(&mut self)
	{
		self.character_index = 0;

		for index in 0..(MAX_CHARS * MAX_LINES)
		{
			let mut character_to_print =  0xF00 | (' ' as CharSize);

			unsafe { *((self.video_output + index * 2) as *mut CharSize) = character_to_print; }
		}
	}

	fn push_to_buffer(&mut self, text: &str, color: VgaColor, new_line: bool)
	{
		for byte in text.bytes()
		{
			unsafe 
			{ 	
				if self.buffer_char_index >= MAX_CHARS
				{
					self.buffer_char_index = 0;

					if self.buffer_line_index < (MAX_LINES -1)
					{ 
						self.buffer_line_index += 1; 
					}
					else 
					{
						self.buffer_line_index = 0; 

						if self.start_print_index < (MAX_LINES -1)
						{							
							self.start_print_index += 1;
						}
						else
						{
							self.start_print_index = 0;							
						}
					}

					let line_index = self.buffer_line_index as usize;
					self.clear_line_buffer(line_index);
				}

				let mut character_to_print = self.vga_color_to_u8(&color) | ( byte as CharSize);
				self.buffer[self.buffer_line_index as usize][self.buffer_char_index as usize] = character_to_print;	

				self.buffer_char_index += 1;				
			}	
		}

		if new_line
		{
			for diff_index in self.buffer_char_index..MAX_CHARS
			{
				let mut character_to_print = self.vga_color_to_u8(&color) | ( ' ' as CharSize);

				self.buffer[self.buffer_line_index as usize][self.buffer_char_index as usize] = character_to_print;	
				self.buffer_char_index += 1; 
			}

			self.buffer_char_index = 0;

			if self.buffer_line_index < (MAX_LINES -1)
			{ 
				self.buffer_line_index += 1; 
				self.start_print_index += 1;
			}
			else 
			{ 
				self.buffer_line_index = 0; 
				self.start_print_index = 0;
			}
		}
	}

	fn print_to_screen(&mut self)
	{
		self.clear_screen();

		for line in self.start_print_index..MAX_LINES
		{
			for character in self.buffer[line as usize].iter()
			{
				unsafe { *((self.video_output + self.character_index * 2) as *mut CharSize) = *character; }

				self.character_index += 1;
			}
		}

		for line in 0..self.start_print_index 
		{
			for character in self.buffer[line as usize].iter()
			{
				unsafe { *((self.video_output + self.character_index * 2) as *mut CharSize) = *character; }

				self.character_index += 1;
			}
		}		
	}

	pub fn print_std(&mut self, text: &str){ self.push_to_buffer(text, VgaColor::White, false); self.print_to_screen(); }
	pub fn print_info(&mut self, text: &str){ self.push_to_buffer(text, VgaColor::Blue, false); self.print_to_screen(); }
	pub fn print_debug(&mut self, text: &str){ self.push_to_buffer(text, VgaColor::Green, false); self.print_to_screen(); }
	pub fn print_warning(&mut self, text: &str){ self.push_to_buffer(text, VgaColor::Yellow, false); self.print_to_screen(); }
	pub fn print_error(&mut self, text: &str){ self.push_to_buffer(text, VgaColor::Red, false); self.print_to_screen(); }
	pub fn print_custom_color(&mut self, text: &str, color: VgaColor){ self.push_to_buffer(text, color, false); self.print_to_screen(); }

	pub fn print_ln_std(&mut self, text: &str){ self.push_to_buffer(text, VgaColor::White, true); self.print_to_screen(); }
	pub fn print_ln_info(&mut self, text: &str){ self.push_to_buffer(text, VgaColor::Blue, true); self.print_to_screen(); }
	pub fn print_ln_debug(&mut self, text: &str){ self.push_to_buffer(text, VgaColor::Green, true); self.print_to_screen(); }
	pub fn print_ln_warning(&mut self, text: &str){ self.push_to_buffer(text, VgaColor::Yellow, true); self.print_to_screen(); }
	pub fn print_ln_error(&mut self, text: &str){ self.push_to_buffer(text, VgaColor::Red, true); self.print_to_screen(); }
	pub fn print_ln_custom_color(&mut self, text: &str, color: VgaColor){ self.push_to_buffer(text, color, true); self.print_to_screen(); }

	pub fn clear(&mut self){ self.clear_buffer(); self.print_to_screen(); }

	pub fn new() -> Self
	{
		Writer
		{
			video_output: 0xb8000,
			character_index: 0,
			buffer: [ [ (0xF00 | (' ' as CharSize)) ; UMAX_CHARS]; UMAX_LINES],
			buffer_line_index: 0,
			buffer_char_index: 0,
			start_print_index: 0
		}
	}
}
