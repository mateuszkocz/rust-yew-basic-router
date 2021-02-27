use yew::prelude::*;
use crate::router::Router;
use crate::route::Route;
use wasm_bindgen::JsValue;

pub struct Root {
    link: ComponentLink<Self>,
    route: String,
}

pub enum Msg {
    ChangeRoute(String),
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Root {
            link,
            route: String::from("/index"),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeRoute(route) => {
                self.route = route
            },
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let create_on_route_click = |route: String| self.link.callback(move |_| Msg::ChangeRoute(route.clone()));
        html! {
            <main>
                <section>
                    <ul>
                        <li><a onclick=create_on_route_click(String::from("/index"))>{"Home"}</a></li>
                        <li><a onclick=create_on_route_click(String::from("/exit"))>{"Exit"}</a></li>
                    </ul>
                </section>
                <Router active=&self.route>
                    <Route key="1" path=String::from("/index")>
                        {"Hello World!"}
                    </Route>
                    <Route key="2" path=String::from("/exit")>
                        {"Good bye World!"}
                    </Route>
                </Router>
            </main>
        }
    }
}
