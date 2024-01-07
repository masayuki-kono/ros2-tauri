// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

struct Publishers {
    operation1: r2r::Publisher<r2r::std_msgs::msg::String>,
    operation2: r2r::Publisher<r2r::std_msgs::msg::String>,
}

#[tauri::command]
fn button1_pushed(message: &str, publishers: tauri::State<Arc<Mutex<Publishers>>>) {
    let msg = r2r::std_msgs::msg::String {
        data: message.to_string(),
    };
    publishers.lock().unwrap().operation1.publish(&msg).unwrap();
}

#[tauri::command]
fn button2_pushed(message: &str, publishers: tauri::State<Arc<Mutex<Publishers>>>) {
    let msg = r2r::std_msgs::msg::String {
        data: message.to_string(),
    };
    publishers.lock().unwrap().operation2.publish(&msg).unwrap();
}

fn main() {
    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "web_ui", "").unwrap();

    let publishers = Arc::new(Mutex::new(Publishers {
        operation1: node
            .create_publisher::<r2r::std_msgs::msg::String>(
                "/operation1",
                r2r::QosProfile::default(),
            )
            .unwrap(),
        operation2: node
            .create_publisher::<r2r::std_msgs::msg::String>(
                "/operation2",
                r2r::QosProfile::default(),
            )
            .unwrap(),
    }));

    std::thread::spawn(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    tauri::Builder::default()
        .manage(publishers)
        .invoke_handler(tauri::generate_handler![button1_pushed, button2_pushed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
