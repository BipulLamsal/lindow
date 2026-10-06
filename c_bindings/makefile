CC = gcc
CFLAGS = -m32 -ffreestanding -nostdlib -fno-builtin -fno-stack-protector -nostartfiles -nodefaultlibs -fno-pic -fno-pie -no-pie -Wall -Wextra -c
LDFLAGS = -m elf_i386 -Ttext 0x1000 --oformat binary -e main 

build:
	$(CC) $(CFLAGS) kernel.c -o kernel.o

bin: kernel.o
	ld $(LDFLAGS) -o kernel.bin kernel.o

asm: kernel.bin
	ndisasm -b 32 kernel.bin > kernel.dis

image: bin
	nasm ../bootloader/bootloader.asm -f bin -o ../bootloader/bootloader.bin -I ../bootloader/
	cat ../bootloader/bootloader.bin kernel.bin > os-image.bin
	truncate -s 8192 os-image.bin
	@echo "os-image.bin created: kernel starting @ 0x1000"

run: image
	qemu-system-x86_64 -drive file=os-image.bin,format=raw

clean:
	rm -f kernel.o kernel.bin kernel.dis os-image.bin ../bootloader/bootloader.bin
