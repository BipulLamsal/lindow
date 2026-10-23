;
; A simple boot sector program that demonstrates segment offsetting
;

mov ah, 0x0e ; int 10/ ah = 0 eh -> scrolling teletype BIOS routine

mov al , [ es : the_secret ] ; Tell the CPU to use the es ( not ds ) segment.
int 0x10

jmp $ ; Jump forever.
the_secret :
db 'A'

; Padding and magic BIOS number.
times 510 -( $ - $$ ) db 0
dw 0xaa55
