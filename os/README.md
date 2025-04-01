
Running a bare minimun architecture, we need a rust target triple used for embedded systems which simply is ARM based no underlying OS embedded application binary interface
```
# installing via rustup
rustup target add thumbv7em-none-eabihf

# building the target file
cargo build --target thumbv7em-none-eabihf


#passing a argument to the linker
cargo rustc -- -C link-arg=-nostartfiles

#runnign direclty on to the host machine (linux)
cargo rustc -- -C link-arg=-nostartfiles

```


Running the os
```
qemu-system-x86_64 -drive file=./target/target/debug/bootimage-os.bin

```
