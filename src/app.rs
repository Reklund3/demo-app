use crate::components::check_for_updates::CheckForUpdates;
use crate::components::create_post::CreatePost;
use crate::components::footer::Footer;
use crate::components::post_details::PostDetails;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            // <PostDetails />
            //
            // <CreatePost />

            <CheckForUpdates />

            <Footer />
        </main>
    }
}
//  d8231da2-293f-4a87-b5a7-d0273ec14f0d
