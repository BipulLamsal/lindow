```rust

trait Animal{
    fn speak(&self);
}

struct Dog;
struct Cat;
impl Animal for Dog {
    fn speak(&self) {
println!("whoof");
}
}

impl Animal for Cat {
fn speak(&self) {
println!("meow");
}
}

```

```
// static dispatch
fn make_it_speak<T:Animal>(animal: T) {
animal.speak();
}

// more idomatic way
fn make_it_speak(animal: impl Animal) {
animal.speak();
}

```


```
// dynamic dispatch
fn make_it_speak(animal:&dyn Animal)  {
animal.speak();
}

```
