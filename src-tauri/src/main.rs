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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = hyper::Client::builder().http2_only(true).build_http();

    let post_svc = tower::ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(client);

    let mut client = post_service::post_service_client::PostServiceClient::with_origin(
        post_svc,
        "http://127.0.0.1:8080".try_into()?,
    );

    let request = tonic::Request::new(post_service::GetPostRequest {
        id: "00768482-f3ea-4a0e-b18d-d3008c838212".to_string(),
    });

    let response = client.get_post(request).await?;

    println!("RESPONSE={:?}", response);

    let request_two = tonic::Request::new(post_service::GetPostsRequest {
        page_number: 0,
        page_size: 25,
    });
    let response_two = client.get_posts(request_two).await?;

    println!("RESPONSE={:?}", response_two);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
