all: run

clean:
	rm -rf out
	rm -rf target

asm: clean
	mkdir -p out
	nasm -f elf64 src/asm/multiboot_header.asm -o out/multiboot_header
	nasm -f elf64 src/asm/boot.asm -o out/boot
	nasm -f elf64 src/asm/long_mode_init.asm -o out/long_mode_init

rust: asm
	cargo build
	cargo bootimage

link: rust
	ld -n -T src/asm/linker.ld -o out/vmlinux \
		out/multiboot_header \
		out/boot out/long_mode_init \
		target/x86_64-unknown-none/debug/dk

iso: link
	mkdir -p out/iso/boot/grub
	cp -r iso out/
	cp out/vmlinux out/iso/boot/
	grub-mkrescue -o out/dk.iso out/iso
	
run: iso
	qemu-system-x86_64 -drive format=raw,file=/home/drazisil/github/dk/target/x86_64-unknown-none/debug/bootimage-dk.bin
	# qemu-system-x86_64 -cdrom out/dk.iso

.PHONY: all clean asm rust link iso run