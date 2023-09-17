// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tonic_web::GrpcWebClientLayer;

pub mod post_service {
    tonic::include_proto!("posts");
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = hyper::Client::builder().build_http();

    let post_svc = tower::ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(client);

    let mut client = post_service::post_service_client::PostServiceClient::with_origin(post_svc, "http://127.0.0.1:8080".try_into()?);


    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
