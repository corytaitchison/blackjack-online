use crate::components::chooser;
use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    S,
    H,
    D,
    T(usize),
    Y,
    N,
}

impl Default for State {
    fn default() -> Self {
        Self::S
    }
}

pub struct ActionButton {
    state: State,
    link: ComponentLink<Self>,
    onsignal: Callback<chooser::Msg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    CycleState,
    ButtonPressed,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    #[props(required)]
    pub state: State,
    #[props(required)]
    pub onsignal: Callback<chooser::Msg>,
}

impl Component for ActionButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: props.state,
            link,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CycleState => {
                self.state = match self.state {
                    State::S => State::H,
                    State::H => State::D,
                    State::D => State::S,
                    State::T(i) => State::T(i),
                    State::Y => State::N,
                    State::N => State::Y,
                }
            }
            Msg::ButtonPressed => {
                self.onsignal.emit(chooser::Msg::None);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.state = props.state;
        self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        html! {
            <button class=("btn-3d", "active", "btn-action", {
                match self.state {
                    State::S => "red",
                    State::H => "blue",
                    State::D => "green",
                    State::T(_) => "yellow",
                    State::Y => "green",
                    State::N => "red",
                }
            }), onclick=self.link.callback(|_| Msg::ButtonPressed),>{ match self.state {
                State::T(99) | State::T(1) => "A".to_string(),
                State::T(i) => format!("{}", i),
                i => format!("{:?}", i),
            }} </button>
        }
    }
}
