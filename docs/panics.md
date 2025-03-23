crazy stuffs with panics in  rust and how it handles in briefly
1. Stack Unwinding: They are simply stacks that grows downwards stack unwinding process goes from top to bottom scanning if any panic is occuring
    if any occurs it checks with personality routine.
2. Perosnality Routine: It checks if the panic should be caught on the current function of the stack, ensuring proper cleanup (landing pads) before
    further moving to the unwinding process of the stack

These are compiler-generated functions for unwinding:
    
    - eh_personality → The personality routine that decides if an exception should be caught and handles stack cleanup.
    - eh_unwind_resume → Controls how unwinding resumes after a cleanup phase, depending on platform-specific settings.
 ! -> this type is known as diverging and doesn't return anything can be used as the type of panic returning

