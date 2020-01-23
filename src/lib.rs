#![recursion_limit = "2048"]

mod blackjack;
mod components;

use self::components::{
    controls::Controls,
    messages::Messages,
    {chooser, chooser::Chooser},
};
use yew::{prelude::*, services::ConsoleService};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Welcome,
    Choosing(chooser::State),
    Scores(usize),
    None,
}

impl Default for State {
    fn default() -> Self {
        Self::None
    }
}

pub struct Model {
    state: State,
    link: ComponentLink<Self>,
    console: ConsoleService,
    messages: Vec<String>,
    chooser_link: Option<ComponentLink<Chooser>>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    ChangeState(State),
    PushMessage(String),
    AssignLink(ComponentLink<Chooser>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            state: State::Welcome,
            link,
            messages: vec![
                "Welcome to Blackjack Online!".to_string(),
                "It's just like normal blackjack except I've taken the fun away because you don't actually play the game!".to_string(),
                "Instead, you just tell the computer what you would do in each situation.".to_string(),
                "Then we run about 13 million simulations and see how much you would've earnt!".to_string(),
            ],
            console: ConsoleService::new(),
            chooser_link: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeState(target) => {
                self.state = target;
                // self.console.log(&format!("{:?}", self.state)[..]);
                match target {
                    State::Scores(outcome) => {
                        self.messages = vec![
                            "Good job!!".to_string(),
                            format!("Score: {}", outcome as isize - 1_000_000isize),
                        ];
                    }
                    State::Welcome => {
                        self.messages = vec![
                            "Welcome to Blackjack Online!".to_string(),
                            "It's just like normal blackjack except I've taken the fun away because you don't actually play the game!".to_string(),
                            "Instead, you just tell the computer what you would do in each situation.".to_string(),
                            "Then we run about 13 million simulations and see how much you would've earnt!".to_string(),
                        ];
                    }
                    _ => (),
                }
            }
            Msg::PushMessage(msg) => {
                self.messages.push(msg);
            }
            Msg::AssignLink(comp) => {
                self.chooser_link = Some(comp);
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="model",>
                <div class=("container-header", "container", "curved"),>
                    <h class="header">{ "💰💰💰 Blackjack Simulator 💰💰💰" }</h>
                </div>
                <div class=("container-main", "container", "curved"),>
                    {
                        match self.state {
                            State::Choosing(choose_state) => html! {
                                <Chooser: state=&choose_state, onsignal=self.link.callback(|comp| Msg::AssignLink(comp)) on_end_game=self.link.callback(|msg| msg),/>
                            },
                            _ => html! {
                                <div class=("container-feature", "container"),>
                                    <Messages: messages=&self.messages,/>
                                </div>
                            }
                        }
                    }
                    <Controls: state=&self.state, onsignal=self.link.callback(|msg| msg) chooser_link=match &self.chooser_link.as_ref() {
                        &None => None,
                        &Some(comp) => Some(comp.clone())
                    },/>
                </div>
            </div>
        }
    }
}
