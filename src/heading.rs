use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub on_click: Callback<()>,
    pub text: String,
    pub children: Children,
}

pub struct Heading {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    ChangeClicked,
}

impl Component for Heading {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Heading {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeClicked => {
                self.props.on_click.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <header>
                {&self.props.text}
                <button onclick=self.link.callback(|_| Msg::ChangeClicked)>{"change"}</button>
                <hr/>
                {self.props.children.clone()}
            </header>
        }
    }
}
