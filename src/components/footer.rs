use yew::{html, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container footer">
                <hr class="row" style="width: 95%;" />

                <p style="margin: 0;">{"This app uses the following technologies."}</p>

                <div class="row">
                    <a href="https://tauri.app" target="_blank">
                        <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                    </a>
                    <a href="https://yew.rs" target="_blank">
                        <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                    </a>
                </div>
            </div>
        }
    }
}
