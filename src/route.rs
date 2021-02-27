use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub path: String,
    pub children: Children,
}

pub struct Route {
    pub props: Props,
}

impl Component for Route {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>{self.props.children.clone()}</div>
        }
    }
}
