```
------------------------
|    Function Args     |  ← Arguments passed to function
------------------------
|  Return Address      |  ← Where to return after function
------------------------
|  Saved Registers     |  ← Caller-saved & callee-saved registers
------------------------
|  Local Variables     |  ← Function-specific storage
------------------------
|  Red Zone (128B)     |  ← Temporary storage (user-mode only)
------------------------
|     RSP →            |  ← Stack pointer (points to top of stack)
------------------------
|  Stack Growth ↓      |
------------------------
Low Memory Addresses   ↓

```

- stack grows downward if i want to use stack without actually moving the RSP (stack pointer) than i need to use negative offset to accccess the read zone.
- The red zone is an optimization introduced by the x86-64 System V ABI to reduce stack pointer manipulations (RSP adjustments) in function calls.
