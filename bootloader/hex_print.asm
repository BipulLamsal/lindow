; Input:
;   DX = value to print
hex_print:
    pusha
    
    push dx
    mov  dx, 12 

lets_go:
    pop  ax              ; get original value
    push ax              ; preserve it for the next iteration

    mov  cl, dl
    shr  ax, cl
    and  ax, 0x000F      

    sub  dx, 4           ; next shift amount
    
    push ax 
    call get_char

    push dx 
    mov  ah, 0x0E
    int  0x10
    pop  dx

    pop  ax 

    cmp  dx, 0
    jge  lets_go

    pop  ax              ; remove preserved original value
    popa
    ret


get_char:
    cmp al, 10
    jl  less

greater:
    add al, 'A' - 10
    ret

less:
    add al, '0'
    ret
