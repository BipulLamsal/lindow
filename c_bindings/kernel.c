/*
 * 0x1000 on the memory, which is the main fucntion of our
 * First C binding i.e main no crt is here 
 * The entry point is set by ld as main  
 * */
void main() {
    char* vga = (char *) 0xb8000;
    vga[0] = 'X';
    vga[1] = 0x0F; /* white on black attribute */
    for (;;) { __asm__ volatile ("hlt"); }
}
