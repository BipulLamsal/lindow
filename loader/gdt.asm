gdt_start:

gdt_null:
    dd 0x0
    dd 0x0

gdt_code:
    dw 0xffff       ; limit
    dw 0x0          ; base (bits 0-15)
    db 0x0          ; base (bits 16-23)
    db 10011010b    ; access byte
    db 11001111b    ; flags (0xC) and limit high value (0xF) see oswiki
    db 0x0          ; base (bits 24-31)

gdt_data:
    dw 0xffff 
    dw 0x0
    db 0x0
    db 10010010b; changes here 
    db 11001111b
    db 0x0 

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1 
    dd gdt_start

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start
