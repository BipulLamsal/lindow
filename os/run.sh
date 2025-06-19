# ( cargo bootimage) &
# ( qemu-system-x86_64 -drive file=./target/target/debug/bootimage-os.bin ) & 
# ( vncviewer localhost:5900 ) & 
# wait

echo -e "Compiling Lindow"
cargo run | tee output.log & CHILD_PID=$!

while ! grep -q "Running" output.log; do
    sleep 1
done


