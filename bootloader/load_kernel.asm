[BITS 16]
load_kernel:
    pusha
    mov ax, 0
    mov es, ax
    mov bx, KERNEL_OFFSET 
    mov dh, 15 ;; 15 sectors 15 * 512
    mov dl, [BOOT_DRIVE]
    call disk_load
    popa
    ret
