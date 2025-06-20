BITlS 16 ; 16 bit real mode 
ORG 0x7C00 ; BIOS loads bootloader before  

;=>PC Memory map at bootup (from OSD)
;===================
;000000-0003FF interrupt vector table
;000400-0004FF BIOS data area
;000500-007BFF FREE CONVENTIONAL MEMORY
;007C00-007DFF boot sector
;007E00-09FBFF FREE CONVENTIONAL MEMORY
;09FC00-09FFFF extended BDA (variable length)
;0A0000-0FFFFF video memory and BIOS ROMs
;100000-10FFEF high memory area (HMA)
;10FFF0- FREE EXTENDED MEMORY start:


start:
; Text mode. 80x25. 16 colors. 8 pages.
mov al, 3
mov ah, 0
int 10h



