IDT : (Interrupt Descriptor Table)
- Knows where to jump when exeption occurs
- Binary Datastructure
- IDT entrites are called gates. It contains Interrupt Gates, Task Gates, Trap Gates
- The location of the IDT is kept in IDTR (IDT register). 
- GDT - Global  Descriptor Table 
- LDT - Local Descriptor Table (Private segement in the memory)
- struct InterruptDescriptor32 {
   uint16_t offset_1;        // offset bits 0..15
   uint16_t selector;        // a code segment selector in GDT or LDT
   uint8_t  zero;            // unused, set to 0
   uint8_t  type_attributes; // gate type, dpl, and p fields
   uint16_t offset_2;        // offset bits 16..31
};

- Type of Interrupt: 
    1. Exception : Traps(Saving the address for the later retires) 
    2. Events : (Persistent to saving the request of the next instruction so CPU can continue later)

    The difference is that TRAP request doesn't halt other intterupts it can still come to CPU, but hardware intterupts requires the End of Inter   rupt (EOI) to allow other intterupts.


- Interrupt Service Routine: 
    - special function to handle the intterupts, CPU pauses the execution and searches IDT for the Int opcode then executes  
