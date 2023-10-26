use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeCheckForUpdate, catch)]
    async fn check_for_update() -> Result<JsValue, JsValue>;
}

#[derive(Properties, PartialEq)]
pub(crate) struct CheckForUpdatesProps;

#[function_component(CheckForUpdates)]
pub(crate) fn check_for_new_version(_check_for_updates_props: &CheckForUpdatesProps) -> Html {
    let check_version = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            spawn_local(async move {
                match check_for_update().await {
                    Ok(response) => println!(
                        "Received update response of {}",
                        response.as_string().unwrap()
                    ),
                    Err(e) => println!("ERROR WHILE UPDATING! {}", e.as_string().unwrap()),
                };
            })
        })
    };

    html! {
        <div>
            <button onclick={check_version}>{"Check for updates..."}</button>
        </div>
    }
}
