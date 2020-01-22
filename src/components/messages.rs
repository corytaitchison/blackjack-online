use yew::prelude::*;

pub struct Messages {
    messages: Vec<String>,
}

pub enum Msg {}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub messages: Vec<String>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

impl Component for Messages {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Messages {
            messages: props.messages,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.messages = props.messages;
        true
    }

    fn view(&self) -> Html {
        let view_message = |message: &String| {
            html! {
                <li>{message}</li>
            }
        };
        html! {
            <div class=("container-messages", "container"),>
                <div class="scroller-wrapper",>
                    <div class="scroller",>
                        <ul>{
                            for self.messages.iter().rev().map(view_message)}
                        </ul>
                    </div>
                </div>
            </div>
        }
    }
}
