use crate::{
    components::{chooser, chooser::Chooser},
    State,
};
use yew::prelude::*;

pub struct Controls {
    state: State,
    onsignal: Callback<crate::Msg>,
    link: ComponentLink<Self>,
    chooser_link: Option<ComponentLink<Chooser>>,
}

pub enum Msg {
    ButtonPressed(crate::Msg),
    PlayGame,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    #[props(required)]
    pub onsignal: Callback<crate::Msg>,
    #[props(required)]
    pub state: State,
    #[props(required)]
    pub chooser_link: Option<ComponentLink<Chooser>>,
}

impl Component for Controls {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Controls {
            state: props.state,
            onsignal: props.onsignal,
            link,
            chooser_link: props.chooser_link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ButtonPressed(msg) => self.onsignal.emit(msg),
            Msg::PlayGame => match &self.chooser_link {
                Some(comp) => comp.callback(|_| chooser::Msg::PlayGame).emit(()),
                None => panic!("Link not found"),
            },
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.state = props.state;
        self.onsignal = props.onsignal;
        self.chooser_link = props.chooser_link;
        true
    }

    fn view(&self) -> Html {
        use crate::Msg::*;
        let buttons = match self.state {
            State::Welcome => html! {<>
                <span>
                    <button class=("btn-3d", "green", "inactive"),>{ " Code " }</button>
                </span>
                <span>
                    <button class=("btn-3d", "blue", "inactive"),>{ "High Scores" }</button>
                </span>
                <span>
                    <button class=("btn-3d", "purple", "active"), onclick=self.link.callback(move |_| Msg::ButtonPressed(ChangeState(State::Choosing(chooser::State::Hard)))),>{ " Play "}</button>
                </span>
            </>},
            State::Choosing(choose_state) => {
                let prev_state = match choose_state {
                    chooser::State::Soft => chooser::State::Hard,
                    chooser::State::Splits => chooser::State::Soft,
                    _ => chooser::State::None,
                };
                let next_state = match choose_state {
                    chooser::State::Hard => chooser::State::Soft,
                    chooser::State::Soft => chooser::State::Splits,
                    _ => chooser::State::None,
                };
                html! {
                    <>
                    <span>
                        <button class=("btn-3d", "green", "active"), onclick=self.link.callback(move |_| Msg::ButtonPressed(ChangeState(State::Welcome))),>{ " Main Menu "}</button>
                    </span>
                    <span>
                    {
                        match prev_state {
                            chooser::State::Hard | chooser::State::Soft => html! {
                                <button class=("btn-3d", "blue", "active"), onclick=self.link.callback(move |_| Msg::ButtonPressed(ChangeState(State::Choosing(prev_state)))),>{ " < "}</button>
                            },
                            _ => html! {<span />}
                        }
                    }
                    </span>
                    <span>
                    {
                        match next_state {
                            chooser::State::Soft | chooser::State::Splits => html! {
                                <button class=("btn-3d", "blue", "active"), onclick=self.link.callback(move |_| Msg::ButtonPressed(ChangeState(State::Choosing(next_state)))),>{ " > "}</button>
                            },
                            _ => html! {
                                <button class=("btn-3d", "purple", "active"), onclick=self.link.callback(move |_| Msg::PlayGame),>{ " Run "}</button>
                            }
                        }
                    }
                    </span>
                    </>
                }
            }
            _ => html! {
                <>
                <span>
                </span>
                <span>
                    <button class=("btn-3d", "purple", "active"), onclick=self.link.callback(move |_| Msg::ButtonPressed(ChangeState(State::Welcome))),>{ " Main Menu "}</button>
                </span>
                <span>
                </span>
                </>
            },
        };
        html! {
            <div class=("container-controls", "container"),>
                { buttons }
            </div>
        }
    }
}
