use crate::prelude::Result;

mod error;
mod prelude;

pub fn lib_hello() {
    println!("Hello from src/lib.rs");
}

pub fn greet() -> Result<()> {
    println!("Hello World!");
    Ok(())
}

#[allow(dead_code)]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
