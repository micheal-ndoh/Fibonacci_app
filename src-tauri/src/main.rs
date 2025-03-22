use tauri::command;
use std::num::ParseIntError;

#[command]
fn calculate_fibonacci(n: u32) -> Result<u32, String> {
    if n == 0 {
        return Ok(0);
    } else if n == 1 {
        return Ok(1);
    }

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }

    Ok(c)
}

#[tauri::main]
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate_fibonacci])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}