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

[BITS 32]
begin_pm:
    ;; we are in code segment (CS.D = 1, so these MUST be assembled as 32-bit)
    ;; now what we do is
    mov ax, VGS_SEG   
    mov es, ax
    
    mov al, 'H'
    mov bl, 0x0F

    mov [es:0x00], al
    mov [es:0x01], bl 
    
    jmp $

[BITS 16]
%include "setup.asm"
%include "print_string.asm"
%include "gdt.asm"
%include "pm.asm"

times 510 - ($ - $$) db 0
db 0x55
db 0xAA
