all: run

asm:
	mkdir -p out
	nasm -f elf64 src/asm/multiboot_header.asm -o out/multiboot_header
	nasm -f elf64 src/asm/boot.asm -o out/boot
	nasm -f elf64 src/asm/long_mode_init.asm -o out/long_mode_init
	ld -n -T src/asm/linker.ld -o out/vmlinux out/multiboot_header out/boot out/long_mode_init

iso: asm
	mkdir -p out/iso/boot/grub
	cp -r iso out/
	cp out/vmlinux out/iso/boot/
	grub-mkrescue -o out/dk.iso out/iso

run: iso
	qemu-system-x86_64 -cdrom out/dk.iso

.PHONY: all iso run