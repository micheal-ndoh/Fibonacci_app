// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn fibonacci(numbers: Vec<u32>) -> Vec<u64> {
    numbers.into_iter().map(|n| {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let temp: u64 = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    }).collect()
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fibonacci])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
