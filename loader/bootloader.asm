[BITS 16]
[ORG 0x7C00]

mov sp, 0x9000
mov bp, sp

mov dh, 10
mov dl, 0
mov bx, SUCCESS_MSG
mov cx, SUCCESS_MSG_LEN
CALL print_string
CALL switch_to_pm 

begin_pm:
    jmp $

%include "setup.asm"
%include "print_string.asm"
%include "gdt.asm"
%include "pm.asm"


times 510 - ($ - $$) db 0
db 0x55
db 0xAA
