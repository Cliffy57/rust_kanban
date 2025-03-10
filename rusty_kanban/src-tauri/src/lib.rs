// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn submitTask(name: &str){
    println!("I was invoked from JavaScript! the task is {}", name);
}
#[tauri::command]
fn update_task(id: &str, text: String) {
    println!("Updating task {} with new text: {}", id, text);
    // Implement actual logic to update the task in your data store (e.g., SQLite, file, etc.)
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![submitTask, update_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
