[BITS 16]
[ORG 0x7C00]

mov [BOOT_DRIVE], dl ;; bios stores our boot drive in dl 

xor ax, ax
mov ds, ax
mov es, ax
mov ss, ax

mov sp, 0x9000
mov bp, sp

mov dh, 10
mov dl, 0

mov bx, SUCCESS_MSG
mov cx, SUCCESS_MSG_LEN

CALL print_string
CALL load_kernel
CALL switch_to_pm
    

[BITS 16]
%include "setup.asm"
%include "print_string.asm"
%include "disk_load.asm"
%include "gdt.asm"
%include "pm.asm"
%include "load_kernel.asm"

times 510 - ($ - $$) db 0
db 0x55
db 0xAA
