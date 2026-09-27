[ bits 16]
switch_to_pm:
    cli ;; clear interupt
    lgdt[gdt_descriptor]

    ;; well move the cr0 1st bit
    mov eax, cr0
    or eax, 0x1
    mov cr0, eax

    jmp CODE_SEG:start_pm


[ bits 32]
start_pm:
    mov ax, DATA_SEG   
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ;; maybe 90000 better here?
    mov sp, 0x9000
    mov bp, sp

    call begin_pm 
