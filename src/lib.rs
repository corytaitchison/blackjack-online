mod components;

use self::components::{controls::Controls, dashboard::Dashboard};
use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    PreGame,
    Betting,
    Playing,
    EndGame,
    None,
}

impl Default for State {
    fn default() -> Self {
        Self::None
    }
}

pub struct Model {
    balance: usize,
    state: State,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    ChangeState(State),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            balance: 4_500,
            state: State::PreGame,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeState(target) => {
                self.state = target;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="model",>
                <div class=("container-header", "container", "curved"),>
                    <h class="header">{ "💰💰💰 Blackjack Online 💰💰💰"}</h>
                </div>
                <div class=("container-main", "container", "curved"),>
                    <div class="main",>
                        <Controls: state=&self.state, onsignal=self.link.callback(|msg| msg),/>
                    </div>
                    <Dashboard: balance={ self.balance },/>
                </div>
            </div>
        }
    }
}
