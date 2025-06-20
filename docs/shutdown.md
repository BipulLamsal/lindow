inorder to shutdown computer uses some sort of power management
either
1. APM
    APM is specifications developed by microsoft and intel. It defines how bios and os should interact to manage power.
    APM is used for single cpu single thread processor, this is realivetly old concept that shuts power or reduces to different component by the
    communications between teh bios firmware and the os. when hardware goes idel the power is reduced or shutddown.

APM bios exposes APIs functions to the OS allowing it to ask the BIOS to:
    - Power down devices
    - Suspend/resume the system
    - Halt CPUs, etc
These APIs can be accessed through different CPU modes
CPU modes are different ways the processor can operate.

Interface means how the os talks to the APM BIOS, which CPU mode teh communications uses. 
    
    1. Real Mode Interface:
        OS talks to Bios as BIOS itself runs in the real mode.
        oldest 16 bit no memory protection, direct access to the
        hardware. Used by BIOS and DOS.
    2. 16 bits  protected mode interface:
        has memory protection and multitasking
        meanwhile os talks in 16 or 32 bit protected modes.
    3. 32 bits protected mode interface

The os and bios needs to speek the same language (use the same CPU mode context) when calling APM function. APM specifies different interface adapted to these modes so OS can communicate power commands without breaking the cpu rules.

WHY not just one intereface?
becausse bios is hardware level firmware and it doesn't know what mode the os is using. so it provides multiple interefaces to be compatilble with multiple generations of OSes.
        
only one itereface can be active at a time.
2. ACPI
