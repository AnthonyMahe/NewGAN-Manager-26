// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod parsing;
mod mapping;
mod logging;

#[tauri::command]
fn generate_mapping(config: mapping::MappingConfig) -> Result<(), String> {
    logging::setup_logging()?; // Setup log on first run
    mapping::generate_mapping(config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![generate_mapping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
