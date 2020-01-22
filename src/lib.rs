#![recursion_limit = "256"]

mod blackjack;
mod components;

use self::components::{controls::Controls, dashboard::Dashboard, messages::Messages};
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
    messages: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    ChangeState(State),
    PushMessage(String),
    PlayGame,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            balance: 4_500,
            state: State::PreGame,
            link,
            messages: vec![
                "Welcome to Blackjack Online!".to_string(),
                "© Cory Aitchison 2020".to_string(),
            ],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeState(target) => {
                self.state = target;
            }
            Msg::PushMessage(msg) => {
                self.messages.push(msg);
            }
            Msg::PlayGame => {
                let outcome = blackjack::play();
                self.update(Msg::PushMessage(format!("${}", outcome)));
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="model",>
                <div class=("container-header", "container", "curved"),>
                    <h class="header">{ match self.state {
                        State::PreGame => "💰💰💰 Blackjack Online 💰💰💰",
                        _ => "Play!"
                    }}</h>
                </div>
                <div class=("container-main", "container", "curved"),>
                    <Messages: messages=&self.messages,/>
                    <Controls: state=&self.state, onsignal=self.link.callback(|msg| msg),/>
                    <Dashboard: balance={ self.balance },/>
                </div>
            </div>
        }
    }
}
