use num_bigint::BigUint;
use num_traits::{One, Zero};

#[tauri::command]
fn fibonacci(numbers: Vec<u32>) -> Vec<String> {
    numbers
        .into_iter()
        .map(|n| match n {
            0 => BigUint::zero(),
            1 => BigUint::one(),
            _ => {
                let mut a = BigUint::zero();
                let mut b = BigUint::one();
                for _ in 2..=n {
                    let temp = a + &b;
                    a = b;
                    b = temp;
                }
                b
            }
        })
        .map(|big_num| big_num.to_string()) // Convert BigUint to String for easier handling in the frontend
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fibonacci])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}