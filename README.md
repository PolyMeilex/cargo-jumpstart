# cargo-jumpstart

Util for Rust file scaffolding

# Install
```sh
cargo install jumpstart
```
# Example
```sh
cd ./src/
cargo jumpstart object MyNewStruct
```
Will create `my_new_struct.rs` like this one
```rust
pub struct MyNewStruct{
}

impl MyNewStruct{
    pub fn new() -> Self{
        Self{}
    }
}
```
