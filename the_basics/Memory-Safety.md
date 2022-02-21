### Memory Safety

- Rust Guarantees memory safety at compile time
- As a part if this, variables must be declared and initialized
- Variables that are declared and not initialized will throw errors

So...

``` 
fn main() {
  let enigma: i32; 
  if true {
    enigma = 42; 
  } 
  println!("enigma is {}", enigma); 
}

```
...would result in an error because the compiler does not know what the value of `true` will be at compile time, so there is no _guarantee_ that enigma 
will have a value. 

However...

```
fn main() {
  let enigma: i32; 
  if true {
    enigma = 42; 
  } else {
    enigma = 7; 
  }
    println!("enigma is {}", enigma); 
}

```

...would work since enigma is _guaranteed_ to have a value.


Well...ok...I guess that is fine... 

WHAT!? it is great when compared to C. What if I declared a variable sans initializing it. What happens? Undefined behavior. 
Enigma could equal 1, 2, 900, whatever that instance of the compiler decides. Although the extra lines may be slightly annoying,
ultimately it is worth memory safety.
  
