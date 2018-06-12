echo "Compilation boot.asm"
nasm -felf32 boot.asm

echo "Compilation kernel"
cd kernel
RUST_TARGET_PATH=$(pwd)
xargo build --target=i386-unknown-none
cd ..

echo "Linkage"
ld -m elf_i386 -n -o kernel.bin -T linker.ld boot.o kernel/target/i386-unknown-none/debug/libkernel.a

echo "Création ISO"
mkdir -p isodir/boot/grub
cp kernel.bin isodir/boot/kernel.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o rosty.iso isodir
