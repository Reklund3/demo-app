// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::post_service::create_post_response::Response;
use crate::post_service::post_service_client::PostServiceClient;
use hyper::client::HttpConnector;
use hyper::Client;
use serde::Serialize;
use serde_json::to_value;
use tauri::updater::UpdateResponse;
use tauri::{App, CustomMenuItem, Manager, Menu, MenuItem, Submenu};
use tonic;
use tonic::body::BoxBody;
use tonic::IntoRequest;
use tonic_web::{GrpcWebCall, GrpcWebClientLayer, GrpcWebClientService};

pub mod post_service {
    tonic::include_proto!("posts");
}

#[derive(Debug, Serialize)]
pub struct GetPostError {
    reason: String,
}

#[derive(Debug, Serialize)]
pub struct ServicePost {
    id: String,
    user_id: String,
    body: String,
    // created_date: Timestamp
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
async fn create_post(user_id: &str, body: &str) -> Result<String, String> {
    let client = hyper::Client::builder().http2_only(true).build_http();

    let post_svc = tower::ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(client);

    let mut client: PostServiceClient<
        GrpcWebClientService<Client<HttpConnector, GrpcWebCall<BoxBody>>>,
    > = PostServiceClient::with_origin(
        post_svc,
        "http://127.0.0.1:8080"
            .try_into()
            .expect("failed to make the post service."),
    );

    let request = tonic::Request::new(post_service::CreatePostRequest {
        user_id: user_id.to_string(),
        body: body.to_string(),
    });

    let result: Result<String, String> = match client.create_post(request).await {
        Ok(r) => {
            let response = r.into_inner().response.unwrap();
            match response {
                Response::Success(r) => {
                    Ok(to_value(ServicePost {
                        id: r.post.clone().unwrap().id,
                        user_id: r.post.clone().unwrap().user_id,
                        body: r.post.clone().unwrap().body,
                        // created_date: post.created_date.unwrap()
                    })
                    .unwrap()
                    .to_string())
                }
                Response::Failure(_ex) => {
                    // Err(to_value(GetPostError {
                    //     reason: ex.errors.to_string(),
                    // })
                    //     .unwrap()
                    //     .to_string())
                    Err(to_value(format!(
                        "failed to create post, will generate a better response message."
                    ))
                    .unwrap()
                    .to_string())
                }
            }
        }
        Err(s) => Err(to_value(format!(
            "failed to create post due to: {} -> {}",
            s.code(),
            s.message()
        ))
        .unwrap()
        .to_string()),
    };
    println!("The result of the create was {:?}", &result);

    result
}

#[tauri::command(rename_all = "snake_case")]
// async fn get_post(id: &str, mut client: PostServiceClient<GrpcWebClientService<Client<HttpConnector, GrpcWebCall<BoxBody>>>>) -> Result<tonic::Response<post_service::Post>, tonic::Status> {
// async fn get_post(id: &str) -> Result<ServicePost, GetPostError> {
async fn get_post(id: &str) -> Result<String, String> {
    let client = hyper::Client::builder().http2_only(true).build_http();

    let post_svc = tower::ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(client);

    let mut client: PostServiceClient<
        GrpcWebClientService<Client<HttpConnector, GrpcWebCall<BoxBody>>>,
    > = PostServiceClient::with_origin(
        post_svc,
        "http://127.0.0.1:8080"
            .try_into()
            .expect("failed to make the post service."),
    );

    let request = tonic::Request::new(post_service::GetPostRequest { id: id.to_string() });

    let result = match client.get_post(request).await {
        Ok(r) => {
            let post = r.into_inner();
            Ok(to_value(ServicePost {
                id: post.id,
                user_id: post.user_id,
                body: post.body,
                // created_date: post.created_date.unwrap()
            })
            .unwrap()
            .to_string())
        }
        Err(s) => Err(to_value(GetPostError {
            reason: s.message().clone().to_string(),
        })
        .unwrap()
        .to_string()),
    };

    result
}

#[tauri::command(rename_all = "snake_case")]
async fn check_for_update(app_handle: tauri::AppHandle) -> Result<String, String> {
    app_handle.trigger_global(tauri::updater::EVENT_CHECK_UPDATE, None);
    Ok("Update Requested".into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // here `"check for update".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let check_for_update_menu_item =
        CustomMenuItem::new("check for update".to_string(), "Check for update...");
    let submenu = Submenu::new("Help", Menu::new().add_item(check_for_update_menu_item));
    let menu = Menu::new().add_submenu(submenu);

    let app = tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "check for update" => {
                event
                    .window()
                    .trigger_global(tauri::updater::EVENT_CHECK_UPDATE, None);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            create_post,
            get_post,
            check_for_update
        ])
        // .build(tauri::generate_context!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
