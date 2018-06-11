nasm -felf32 boot.asm
ld -m elf_i386 -n -o kernel.bin -T linker.ld boot.o
mkdir -p isodir/boot/grub
cp kernel.bin isodir/boot/kernel.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o rosty.iso isodir
