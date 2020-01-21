use yew::prelude::*;

pub struct Dashboard {
    balance: usize,
}

pub enum Msg {}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub balance: usize,
}

impl Default for Props {
    fn default() -> Self {
        Self { balance: 5_000 }
    }
}

impl Component for Dashboard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Dashboard {
            balance: props.balance,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.balance = props.balance;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=("container", "dashboard"),>
                <p>{format!("Balance: ${}", self.balance)}</p>
                <p>{ "Test! "}</p>
                <p>{ "Test! "}</p>
            </div>
        }
    }
}
