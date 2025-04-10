x86 -> 32 bit (32 bits register) computers the vary first 8086 (16bits)
x64 -> 64 bit registers in use (2^64) memory can be used theoritical 

bios and uefi firmware standards

real mode 16 bit compatibility mode
protected mode 32 bit compatibility mode
long mode 64 bit

when computer gets turned on
1. BIOS is loaded from special flash memory located on the motherboard
2. The BIOS runs (POST) self-test and initialization routines of the hardware, then it looks for bootable disks.
3. If it finds one, control is transferred to its bootloader, which is a 512-byte portion of executable code stored at the disk’s beginning. 
4. Bootloader has to determine the location of the kernel image and load it in memory
5. switched from real mode to other modes
6. Its third job is to query certain information (such as a memory map) from the BIOS and pass it to the OS kernel.
7. cumerstone is really a nice word to use, same bro bootloader is cumerstone to create so i want to create it 
