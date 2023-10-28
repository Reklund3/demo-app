use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeCreatePost, catch)]
    async fn create_post_callback(user_id: String, body: String) -> Result<JsValue, JsValue>;
}

#[derive(Properties, PartialEq)]
pub(crate) struct CreatePostProps;

#[function_component(CreatePost)]
pub(crate) fn create_post(_create_post_props: &CreatePostProps) -> Html {
    // CREATE POST

    let create_post_body_input_ref = use_node_ref();
    let create_post_body = use_state(|| String::new());

    let create_post_msg = use_state(|| String::new());
    {
        let create_post_msg = create_post_msg.clone();
        let create_post_body = create_post_body.clone();
        let create_post_body2 = create_post_body.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if create_post_body.is_empty() {
                        return;
                    }

                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = match create_post_callback(
                        "8b6a5f15-7e5c-4418-b3c3-672f5c24abb7".to_string(),
                        create_post_body.to_string(),
                    )
                    .await
                    {
                        Ok(response) => response.as_string().unwrap(),
                        Err(error) => error.as_string().unwrap(),
                    };
                    create_post_msg.set(new_msg);
                });

                || {}
            },
            create_post_body2,
        );
    }

    let create_post_on_click = {
        println!("submitted request to create post");
        let create_post_body = create_post_body.clone();
        let create_post_body_input_ref = create_post_body_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            create_post_body.set(
                create_post_body_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <div>
            <form class="row" onsubmit={create_post_on_click}>
                <input id="get-post-input" ref={create_post_body_input_ref} placeholder="Enter the post message" />
                <button type="submit">{"Submit"}</button>
            </form>

            <p><b>{ &*create_post_body }</b></p>
        </div>
    }
}
