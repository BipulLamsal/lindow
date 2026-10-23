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

gdt_vga_code:
    dw 0xffff          ; limit
    dw 0x8000          ; base bits 0-15  (0xB8000 & 0xFFFF = 0x8000)
    db 0x0B            ; base bits 16-23 (0xB8000 >> 16 = 0x0B)
    db 10011010b       ; access byte (code, executable, present, ring0)
    db 11001111b       ; flags limit high
    db 0x0             ; base bits 24-31

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1 
    dd gdt_start

CODE_SEG equ gdt_code - gdt_start ;; 0x08
DATA_SEG equ gdt_data - gdt_start ;; 0x10
VGA_SEG equ gdt_vga_code - gdt_start 
