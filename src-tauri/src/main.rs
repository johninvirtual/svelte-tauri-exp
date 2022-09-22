#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod game_of_life;

use game_of_life::Universe;
use std::sync::Mutex;

#[tauri::command]
fn knock(message: &str) -> String {
    format!("{}, whose there --from rust", message)
}

#[tauri::command]
fn render_game_of_life(universe: tauri::State<Mutex<Universe>>) -> String {
    universe.lock().unwrap().tick();
    return universe.lock().unwrap().render();
}

#[tauri::command]
fn restart_game_of_life(universe: tauri::State<Mutex<Universe>>) {
    universe.lock().unwrap().restart();
}

fn main() {
    let universe = Mutex::new(Universe::new());

    tauri::Builder::default()
        .manage(universe)
        .invoke_handler(tauri::generate_handler![
            knock,
            render_game_of_life,
            restart_game_of_life
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
