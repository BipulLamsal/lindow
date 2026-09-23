[BITS 16]
[ORG 0x7C00]

main:
    mov [BOOT_DRIVE], dl
    mov ax, 0
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    
    mov dh, 5
    mov bx, 0x9000

    mov dl, [BOOT_DRIVE]
    CALL disk_load

    mov dh, 10
    mov dl, 0
    mov bx, SUCCESS_MSG
    mov cx, SUCCESS_MSG_LEN
    CALL print_string
    jmp $


%include "setup.asm"
%include "print_string.asm"
%include "disk_load.asm"

times 510 - ($ - $$) db 0
db 0x55
db 0xAA

; 5 sectors sector padding
times 5*256 dw 0xDADA
