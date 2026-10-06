# C Bindings

Bootloader simply loads the second sector big enough to hold the kernel in the memoery for us its 0x1000.
### Run the bootloader:

```
make run
```

Deassambler:
```
ndisasm -b 32 main.bin > main.dis
```
