pub fn lib_hello() {
    println!("Hello from src/lib.rs");
}

#[allow(dead_code)]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
