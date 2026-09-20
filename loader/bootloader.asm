[BITS 16]
[ORG 0x7C00]

main:
    mov ax, 0
    mov ds, ax
    mov es, ax
    mov ss, ax  
    mov sp, 0x7C00          ; stack grows from bottom to top

    mov ah, 0x01            ; cursor shape
    mov cx, 0x0007
    int 0x10

    call page
    call print_string
    jmp $                   ; stop here

page:
    mov ah, 0x05            ; select active page
    mov al, 0x01            ; page 1
    int 0x10

    mov ah, 0x02            ; set cursor position
    mov bh, 0x01            ; page 1
    mov dh, 0x00            ; row 0
    mov dl, 0x00            ; column 0
    int 0x10
    ret

print_string:
    mov bp, msg             ; ES:BP 
    mov ah, 0x13            ; write string
    mov al, 0x01            ; mode 1: cursor moves
    mov bh, 0x01            ; page 1
    mov bl, 0x07            ; light grey on black
    mov cx, len             ; number of characters
    mov dx, 0x0000          ; row 0, column 0
    int 0x10
    ret

;; cpu isntructon never run this

msg: db 'Hello world', 0
len equ $ - msg - 1

times 510 - ($ - $$) db 0
db 0x55
db 0xAA
