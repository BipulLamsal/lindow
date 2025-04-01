( cargo bootimage) &
( qemu-system-x86_64 -drive file=./target/target/debug/bootimage-os.bin ) & 
( vncviewer localhost:5900 ) & 
wait


