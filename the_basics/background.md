- Systems programming language:
    - Trifecta: Safety, Concurrency, Speed
    - Ruby/Python have safety, not currency or speed
    - C/C++ concurrency and speed, not safety

- Mozilla released rust in 2015
- Mozilla was sick on C++
- Core of quantum written in Rust


- Cargo: Rust's package manager...but wait there is more. It is also the Docs generator, the build system(no makefiles), etc. 


- ```cargo new hello``` will create a hello package(binary apllication)
    - You will get a hello directory, a config file Cargo.toml, and a source subdir with main.rs
        - rs is rust source
        - toml is tom's obvious minimal language

    - Cargo.toml is the source of truth for your application. 
        - Rust uses semantic versioning(three numbers separated by dots)
            - Given a version number MAJOR.MINOR.PATCH, increment the:
                - MAJOR version when you make incompatible API changes,
                - MINOR version when you add functionality in a backwards compatible manner, and
                - PATCH version when you make backwards compatible bug fixes.

- ```cargo run```
    - Command to build and run your project in one step. 
    - if nothing changes in the code, it will not recompile
    - ignore target directories

- if you want to run a release version, use ```cargo run --release```
    - target will goto release, not debug

- damn good compiler...explanations, explanation commands, etc

## Variables

- Rust tends to mimic as many other languages as possible, C and python in particular. 

- To declare a variable, use a ```let``` statement

- Rust is strongly typed. You can annotate types because sometimes rust can not infer it. Do that that like this: ```let people: i32 = 4;``` which signifies a signed 32 bit integer. 

- Let can be used to initiliaze many variables at once
    - ```let (people, cars) = (10, 5);```

- Varaiables are mutable in Rust. Unless you choose to make them immutable, you will never be able to change their value. Why?
    - Safety, concurrency, speed
        - Data that cannot change can be chared amoun multiple threads without locks
        - the compiler can do extra optimizations on data that it knows cannot change
        
- You can make variables mutable though:
    - ```let mut people = 10;``` will allow you to do something like:
        - `fn main() {
            let mut people = 10; 
            people = 2;
        }`

- The ```const``` 
    - screaming snake case
    - type annotation required
    - value must be a constant expression that can be determined at compile time. 
