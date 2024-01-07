// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

#[tauri::command]
fn button_pushed(
    message: &str,
    publisher: tauri::State<Arc<Mutex<r2r::Publisher<r2r::std_msgs::msg::String>>>>,
) {
    let msg = r2r::std_msgs::msg::String {
        data: message.to_string(),
    };
    publisher.lock().unwrap().publish(&msg).unwrap();
}

fn main() {
    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "web_ui", "").unwrap();

    let pub_operation = Arc::new(Mutex::new(
        node.create_publisher::<r2r::std_msgs::msg::String>(
            "/operation",
            r2r::QosProfile::default(),
        )
        .unwrap(),
    ));

    std::thread::spawn(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    tauri::Builder::default()
        .manage(pub_operation)
        .invoke_handler(tauri::generate_handler![button_pushed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
