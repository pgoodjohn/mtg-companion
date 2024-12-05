// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

extern crate r2d2;
extern crate r2d2_sqlite;

mod card;
mod configuration;
mod storage;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let configuration = configuration::Configuration::init().unwrap();
    log::info!("Starting MTG Companion!");

    let db_pool = storage::setup_database(&configuration).expect("Could not set up database.");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(configuration)
        .manage(db_pool)
        .invoke_handler(tauri::generate_handler![
            greet,
            configuration::load_configuration_command,
            card::save_card_command,
            card::load_cards_command,
            card::delete_card_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
