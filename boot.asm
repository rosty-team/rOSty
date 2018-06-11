; Copyright 2018 rOSty team
; This file is part of rOSty.
; rOSty is free software: you can redistribute it and/or modify
; it under the terms of the GNU General Public License as published by
; the Free Software Foundation, either version 3 of the License, or
; (at your option) any later version.
; rOSty is distributed in the hope that it will be useful,
; but WITHOUT ANY WARRANTY; without even the implied warranty of
; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
; GNU General Public License for more details.
; You should have received a copy of the GNU General Public License
; along with rOSty.  If not, see <http://www.gnu.org/licenses/>.

; multiboot2 header
; documentation: https://www.gnu.org/software/grub/manual/multiboot2/
magic equ 0xE85250D6
architecture equ 0
header_length equ header_end - header_start

section .header
header_start:
	dd magic
	dd architecture
	dd header_length
	dd -(magic + architecture + header_length)

	; end tag
	dw 0
	dw 0
	dd 8
header_end:
 
section .text
global start
start:
	; affiche "OK" à l'écran
    mov dword [0xb8000], 0x2f4b2f4f
    hlt
