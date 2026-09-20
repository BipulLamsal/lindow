# Bootloader
- Size of a bootsector is 512 bytes (Master Boot Record) 
- A device is "bootable" if it carries a boot sector with the byte sequence 0x55, 0xAA in bytes 511 and 512 respectively.
- A good practice is to enforce CS:IP at the very start of your boot sector.
- On a hard drive, the so-called Master Boot Record (MBR) holds executable code at offset 0x0000 - 0x01bd, followed by table entries for the four primary partitions, using sixteen bytes per entry (0x01be - 0x01fd), and the two-byte signature (0x01fe - 0x01ff). 
- INT 0x10 is a BIOS video interrupt.
- Allocate an interrupt per BIOS routine, so BIOS
multiplexes the ISRs by what we could imagine as a big switch statement, based usually
on the value set in one of the CPUs general purpose registers, ax, prior to raising the
interrupt.

# References
https://viralpatel.net/taj/tutorial/booting.php
https://wiki.osdev.org/Memory_Map_(x86)#Overview
https://wiki.osdev.org/MBR_(x86)
https://en.wikipedia.org/wiki/BIOS_interrupt_call
https://web.stanford.edu/class/cs107/resources/x86-64-reference.pdf
https://osdev.wiki/wiki/BIOS
https://en.wikipedia.org/wiki/INT_10H
https://angom.myweb.cs.uwindsor.ca/teaching/cs330/WritingOS.pdf
