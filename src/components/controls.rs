use crate::State;
use yew::prelude::*;

pub struct Controls {
    state: State,
    onsignal: Callback<crate::Msg>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ButtonPressed(crate::Msg),
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    #[props(required)]
    pub onsignal: Callback<crate::Msg>,
    pub state: State,
}

// impl Default for Props {
//     fn default() -> Self {
//         Self {
//             onsignal: None,
//             state: State::None,
//         }
//     }
// }

impl Component for Controls {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Controls {
            state: props.state,
            onsignal: props.onsignal,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ButtonPressed(msg) => {
                self.onsignal.emit(msg)
                // if let Some(ref mut callback) = self.onsignal {
                //     callback.emit(msg);
                // }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.state = props.state;
        self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        use crate::Msg::*;
        let s = match self.state {
            State::PreGame => State::Betting,
            _ => State::PreGame,
        };
        html! {
            <div class=("container container-controls"),>
                <div class="top",>
                    <p>{ format!("{:?}", self.state) }</p>
                </div>
                <button class=("btn-3d", "green", "centre"), onclick=self.link.callback(move |_| Msg::ButtonPressed(ChangeState(s))),>{ "👉😎👉 Click me!" }</button>
            </div>
        }
    }
}
