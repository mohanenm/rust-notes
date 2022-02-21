Scope: 

- Begins where it is created and extends to the end of the block. X is defined in the main functions block. Then we define why in a nested block. 
- Once the block ends, Y is dropped. There is no garbage collection at this point. Values are always dropped when they go out of scope. 


``` 
fn main() {
    let x =5; 
    { 
        let y = 99; 
        println!("{}. {}", x, y);
    }
    println!("{}, {}", x, y)l; // error!
}
        
```

- Shadowing the same variable name having different scopes
- Shadowing the same variable in the same scope

```
fn main() {
  let mut x = 5; // X IS MUTABLE
  let x = x; // X IS IMMUTABLE
}
```

- Compiler will usually optimize what is above meaning that nothing will actually happen in the assembly code --> there will be no changes to X's mutability, 
  it will just have the properties that is has at the "end". 
