// filepath: /fibonacci-calculator/fibonacci-calculator/src-tauri/src/fib.rs
use std::error::Error;

#[tauri::command]
pub fn fibonacci(n: u32) -> Result<u32, String> {
    if n < 0 {
        return Err("Input is a negative number. Please enter a non-negative integer.".to_string());
    }
    Ok(fib(n))
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}