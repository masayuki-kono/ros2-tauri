// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures::stream::StreamExt;
use std::sync::{Arc, Mutex};
use tauri::Manager;

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

#[tokio::main]
async fn main() {
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

    let sub_operation_enabled = node
        .subscribe::<r2r::std_msgs::msg::Bool>("/operation_enabled", r2r::QosProfile::default())
        .unwrap();
    let operation_enabled = Arc::new(Mutex::new(false));

    std::thread::spawn(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    tauri::Builder::default()
        .manage(publishers)
        .invoke_handler(tauri::generate_handler![button1_pushed, button2_pushed])
        .setup(move |app| {
            let app_handle = app.handle();

            tokio::spawn(async move {
                sub_operation_enabled
                    .for_each(|msg| {
                        let mut enabled = operation_enabled.lock().unwrap();
                        *enabled = msg.data;
                        let payload = msg.data;
                        let main_window = app_handle.get_window("main").unwrap();
                        main_window
                            .emit("operation-enabled-updated", &payload)
                            .unwrap();
                        futures::future::ready(())
                    })
                    .await
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
