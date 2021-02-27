use yew::prelude::*;
use crate::route::Route;
use wasm_bindgen::JsValue;
use yew::services::ConsoleService;

#[derive(Clone, Properties)]
pub struct Props {
    pub active: String,
    pub children: ChildrenWithProps<Route>,
}

pub struct Router {
    props: Props,
}

impl Component for Router {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Router {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let route = self.get_active_route();
        html! {
            <div>
                <div>
                    {route}
                </div>
            </div>
       }
    }
}

impl Router {
    fn get_active_route(&self) -> Html {
        let items = self.props.children.iter().filter(|route| {
            route.props.path == self.props.active
        }).collect::<Html>();
        html!{{
            items
        }}
    }
}
