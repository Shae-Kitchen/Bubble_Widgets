// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn create_window(
    app: tauri::AppHandle,
    label: String,
    title: String,
    url: String,
    width: f64,
    height: f64,
) -> Result<(), String> {
    use tauri::Manager;
    
    println!("[create_window] START - Creating window: label={}, title={}, url={}", label, title, url);
    println!("[create_window] Dimensions: {}x{}", width, height);
    
    // Check if window already exists
    println!("[create_window] Checking if window '{}' already exists...", label);
    if let Some(window) = app.get_webview_window(&label) {
        println!("[create_window] Window already exists: {}, focusing it", label);
        let _ = window.set_focus();
        println!("[create_window] Returning Ok (existing window)");
        return Ok(());
    }
    println!("[create_window] Window doesn't exist, creating new one...");

    // Create new window
    println!("[create_window] Parsing URL: {}", url);
    let parsed_url = url.parse().map_err(|e| {
        let error_msg = format!("Failed to parse URL: {}", e);
        println!("[create_window] ERROR: {}", error_msg);
        error_msg
    })?;
    
    println!("[create_window] Building window with WebviewWindowBuilder...");
    println!("[create_window] Creating builder instance...");
    
    // Clone values for the closure
    let label_clone = label.clone();
    
    // Spawn window creation on the main thread
    tauri::async_runtime::spawn(async move {
        println!("[create_window] Inside async spawn, creating builder...");
        
        let result = tauri::WebviewWindowBuilder::new(&app, &label_clone, tauri::WebviewUrl::App(parsed_url))
            .title(&title)
            .inner_size(width, height)
            .resizable(true)
            .decorations(false)
            .transparent(true)
            .always_on_top(false)
            .build();
        
        match result {
            Ok(window) => {
                println!("[create_window] SUCCESS - Window created: {}", label_clone);
                println!("[create_window] Window label: {}", window.label());
            }
            Err(e) => {
                println!("[create_window] ERROR building window: {}", e);
            }
        }
    });
    
    println!("[create_window] Spawn initiated, returning Ok");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, create_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
