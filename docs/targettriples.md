GCC & CLANG(LLVM)
GCC : every taget has seperate header, binary files libraries
CLANG: targets multiple devices with jsut -target flag

architecture - vendor - os

CPU architecture, the vendor, the operating system, and the ABI. x86_64-unknown-linux-gnu target triple describes a system with an x86_64 CPU, no clear vendor and a Linux operating system with the GNU ABI. Rust supports many different target triples, including arm-linux-androideabi for Android or wasm32-unknown-unknown for WebAssembly.


