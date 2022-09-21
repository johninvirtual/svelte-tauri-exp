#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn knock(message: &str) -> String {
    format!("{}, whose there --from rust", message)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![knock])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
