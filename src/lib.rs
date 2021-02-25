mod utils;
mod heading;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use heading::Heading;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    title: String,
}

enum Msg {
    AddOne,
    ToggleTitle,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            title: String::from("Hello from the heading!"),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::ToggleTitle => if self.title.eq("Hello from the heading!") {
                self.title = String::from("Now it's a different title.")
            } else {
                self.title = String::from("Hello from the heading!")
            },
        }
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::AddOne);
        let on_heading_click = self.link.callback(|_| Msg::ToggleTitle);
        html! {
            <div>
                <Heading text={&self.title} on_click=on_heading_click>
                    <p>{"Internal content"}</p>
                </Heading>
                <button onclick=onclick>{"+1"}</button>
                <p>{self.value}</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_as_body();
}
