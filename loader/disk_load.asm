; load DH sectors to ES : BX from drive DL
disk_load:
    push dx

    mov ah, 0x02
    mov al, dh
    mov ch, 0x0
    mov cl, 0x02 ;; read from second sector
    mov dh, 0x0

    int 0x13
    pop dx
    
    jc disk_error_fn
    
    cmp dh, al
    jne disk_error_fn
    ret

disk_error_fn:
    mov ax, 0
    mov es, ax 

    mov dh, 10;;; row
    mov dl, 0
    mov bx, DISK_ERROR_MSG
    mov cx, DISK_ERROR_MSG_LEN
    call print_string
    JMP $
