;; mov bx, MSG
;; mov cx, LEN
;; mov dh, ROW
;; mov dl, COL
;; call print_string
print_string:
    mov bp, bx
    mov ah, 0x13
    mov al, 0x01
    mov bh, 0x00
    mov bl, 0x07
    int 0x10
    ret
