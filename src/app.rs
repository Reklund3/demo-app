use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeGreet)]
    async fn greet(name: String) -> JsValue;

    #[wasm_bindgen(js_name = invokeGetPost, catch)]
    async fn get_post(id: String) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct GetPostArgs<'a> {
    id: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg: String = greet(name.to_string()).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
            name2,
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    let get_post_input_ref = use_node_ref();

    let post_id: UseStateHandle<String> = use_state(|| String::new());

    let get_post_msg = use_state(|| String::new());
    {
        let get_post_msg = get_post_msg.clone();
        let post_id = post_id.clone();
        let post_id2 = post_id.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if post_id.is_empty() {
                        return;
                    }
                    // let args = to_value(&GetPostArgs { id: &*post_id }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let result: Result<JsValue, JsValue> = get_post(post_id.to_string()).await;
                    let new_msg = match result {
                        Ok(response) => response.as_string().unwrap(),
                        Err(error) => error.as_string().unwrap(),
                    };
                    get_post_msg.set(new_msg);
                });

                || {}
            },
            post_id2,
        );
    }
    let get_post = {
        println!("submitted request to get post");
        let post_id = post_id.clone();
        let get_post_input_ref = get_post_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            post_id.set(
                get_post_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>

            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <p>
                {"Recommended IDE setup: "}
                <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
                {" + "}
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
                {" + "}
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
            </p>

            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                <button type="submit">{"Greet"}</button>
            </form>

            <p><b>{ &*greet_msg }</b></p>

            <form class="row" onsubmit={get_post}>
                <input id="get-post-input" ref={get_post_input_ref} placeholder="Enter the post id to fetch" />
                <button type="submit">{"Submit"}</button>
            </form>

            <p><b>{ &*get_post_msg }</b></p>
        </main>
    }
}
//  d8231da2-293f-4a87-b5a7-d0273ec14f0d
