// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hyper::Client;
use hyper::client::HttpConnector;
use tonic::body::BoxBody;
use tonic_web::{GrpcWebCall, GrpcWebClientLayer, GrpcWebClientService};
use crate::post_service::post_service_client::PostServiceClient;

pub mod post_service {
    tonic::include_proto!("posts");
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct GetPostError {
    reason: String
}

// #[tauri::command]
// // async fn get_post(id: &str, mut client: PostServiceClient<GrpcWebClientService<Client<HttpConnector, GrpcWebCall<BoxBody>>>>) -> Result<tonic::Response<post_service::Post>, tonic::Status> {
// async fn get_post(id: &str, mut client: PostServiceClient<GrpcWebClientService<Client<HttpConnector, GrpcWebCall<BoxBody>>>>) -> Result <tonic::Response<post_service::Post>, GetPostError> {
//
// }

// async fn get_post_call(id: &str, mut client: PostServiceClient<GrpcWebClientService<Client<HttpConnector, GrpcWebCall<BoxBody>>>>) -> Result <tonic::Response<post_service::Post>, GetPostError> {
async fn get_post_call(id: &str, mut client: PostServiceClient<GrpcWebClientService<Client<HttpConnector, GrpcWebCall<BoxBody>>>>) {
    let request = tonic::Request::new(post_service::GetPostRequest {
        id: id.to_string(),
    });

    let response = client.get_post(request).await;

    println!("RESPONSE={:?}", response.expect(""));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = hyper::Client::builder().http2_only(true).build_http();

    let post_svc = tower::ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(client);

    let mut client: PostServiceClient<GrpcWebClientService<Client<HttpConnector, GrpcWebCall<BoxBody>>>> = PostServiceClient::with_origin(
        post_svc,
        "http://127.0.0.1:8080".try_into()?,
    );

    get_post_call("23c71bef-b92c-437c-a94e-599b03cf7501", client).await;

    // let request_two = tonic::Request::new(post_service::GetPostsRequest {
    //     page_number: 0,
    //     page_size: 25,
    // });
    // let response_two = client.get_posts(request_two).await?;
    //
    // println!("RESPONSE={:?}", response_two);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
