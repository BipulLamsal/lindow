# Macors
rust macors are just piece of code that can generate rust code on itself

example:

// Declarative macros

```rust

macro_rules! nothing {
    ($x:expr) => {};
}

```
this nothign macros does nothing, it accepts the expression and well doesn nothing as name suggest but how to call

call like this
nothing!(//put any random shit here event println!)



lets look at the second exmaple for two arguments required macro repeaeter 
```
macro_rules! repeater {
    ($rep:expr,$valu:expr) => {
        for _ in 0..$rep {
            println!("{:?}", $valu);
        }
    };
}
```


// we can also accpet like keey value pairs and the block itself just fool around and find which works the best for u

// Procedural Macorrs

Procedural macors can be written comlelelty on rust
