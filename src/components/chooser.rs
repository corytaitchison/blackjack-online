use crate::blackjack;
use crate::components::{action_button, action_button::ActionButton, messages::Messages};
use ndarray::{arr2, Array2};
use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Hard,
    Soft,
    Splits,
    None,
}

impl Default for State {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    #[props(required)]
    pub state: State,
    #[props(required)]
    pub onsignal: Callback<ComponentLink<Chooser>>,
    #[props(required)]
    pub on_end_game: Callback<crate::Msg>,
}

pub struct Chooser {
    state: State,
    link: ComponentLink<Self>,
    on_end_game: Callback<crate::Msg>,
    hard_array: Array2<&'static str>,
    soft_array: Array2<&'static str>,
    splits_array: Array2<&'static str>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    ChangeState(State),
    ToggleRow(usize),
    ToggleColumn(usize),
    ToggleCell(usize, usize),
    PlayGame,
    None,
}

impl Component for Chooser {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.onsignal.emit(link.clone());
        Self {
            state: props.state,
            link,
            on_end_game: props.on_end_game,
            hard_array: arr2(&[
                ["s", "d", "d", "d", "d", "s", "s", "s", "s", "s"], // 9
                ["d", "d", "d", "d", "d", "d", "d", "d", "s", "s"], // 10
                ["d", "d", "d", "d", "d", "d", "d", "d", "d", "d"], // 11
                ["h", "h", "s", "s", "s", "h", "h", "h", "h", "h"],
                ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"],
                ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"],
                ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"],
                ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"], //16
                ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"], //17
                ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"], //18
            ]),
            soft_array: arr2(&[
                ["h", "h", "h", "d", "d", "h", "h", "h", "h", "h"], //A,2
                ["h", "h", "h", "d", "d", "h", "h", "h", "h", "h"], //A,3
                ["h", "h", "d", "d", "d", "h", "h", "h", "h", "h"], //A,4
                ["h", "h", "d", "d", "d", "h", "h", "h", "h", "h"],
                ["h", "d", "d", "d", "d", "h", "h", "h", "h", "h"],
                ["d", "d", "d", "d", "d", "s", "s", "h", "h", "h"],
                ["s", "s", "s", "s", "d", "s", "s", "s", "s", "s"],
                ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"],
                ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"], //A,10
                ["h", "h", "h", "d", "d", "h", "h", "h", "h", "h"], //A,A
            ]),
            splits_array: arr2(&[
                ["y", "y", "y", "y", "y", "y", "n", "n", "n", "n"], // 2,2
                ["y", "y", "y", "y", "y", "y", "n", "n", "n", "n"], // 3,3
                ["n", "n", "n", "y", "y", "n", "n", "n", "n", "n"],
                ["n", "n", "n", "n", "n", "n", "n", "n", "n", "n"],
                ["y", "y", "y", "y", "y", "n", "n", "n", "n", "n"],
                ["y", "y", "y", "y", "y", "y", "n", "n", "n", "n"],
                ["y", "y", "y", "y", "y", "y", "y", "y", "y", "y"],
                ["y", "y", "y", "y", "y", "n", "y", "y", "n", "n"],
                ["n", "n", "n", "n", "n", "n", "n", "n", "n", "n"], // 10, 10
                ["y", "y", "y", "y", "y", "y", "y", "y", "y", "y"], // A,A
            ]),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let toggle_item = |s: &mut &str| {
            *s = match *s {
                "s" => "h",
                "h" => "d",
                "d" => "s",
                "y" => "n",
                "n" => "y",
                _ => panic!("Unrecognisable entry!"),
            }
        };
        let active_array = match self.state {
            State::Hard => &mut self.hard_array,
            State::Soft => &mut self.soft_array,
            State::Splits => &mut self.splits_array,
            State::None => panic!("Shouldn't be here"),
        };
        match msg {
            Msg::ChangeState(state) => {
                self.state = state;
            }
            Msg::ToggleCell(i, j) => {
                toggle_item(&mut active_array[[i, j]]);
            }
            Msg::ToggleRow(i) => {
                for j in 0..10 {
                    toggle_item(&mut active_array[[i, j]]);
                }
            }
            Msg::ToggleColumn(j) => {
                for i in 0..10 {
                    if i == 8 && self.state == State::Soft {
                        continue;
                    };
                    toggle_item(&mut active_array[[i, j]]);
                }
            }
            Msg::PlayGame => {
                let outcome =
                    blackjack::play(&self.hard_array, &self.soft_array, &self.splits_array);
                self.on_end_game
                    .emit(crate::Msg::ChangeState(crate::State::Scores(outcome)));
            }
            _ => return false,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.state = props.state;
        true
    }

    fn view(&self) -> Html {
        let convert_state = |s: &str| match s {
            "s" => action_button::State::S,
            "h" => action_button::State::H,
            "d" => action_button::State::D,
            "y" => action_button::State::Y,
            "n" => action_button::State::N,
            _ => panic!("Unrecognisable state"),
        };
        html! {
            <div class=("container", "container-chooser"),>
                <div class=("container", "chooser-info"),>
                    <Messages: messages= match self.state {
                        State::Hard =>  vec![
                            "--- HARD HANDS ---".to_string(),
                            "The rows (numbers 9-18) represent the sum total of the cards in your hand, if you do NOT have an Ace.".to_string(),
                            "The columns (numbers 2-A) represent the dealer's card that is face up at the start of the round.".to_string(),
                            "\"S\" means STAND - don't pick up any more cards.".to_string(),
                            "\"H\" means HIT - take another card from the deck.".to_string(),
                            "\"D\" means DOUBLE - double your starting bet and pick up only one more card.".to_string(),
                            "Choose your action by clicking the buttons on the right.".to_string(),
                            "The yellow buttons toggle the entire row / column.".to_string(),
                        ],
                        State::Soft => vec![
                            "--- SOFT HANDS ---".to_string(),
                            "The rows (numbers 2-A) represent the other card if one of your cards is an Ace.".to_string(),
                            "The columns (numbers 2-A) represent the dealer's card that is face up at the start of the round.".to_string(),
                            "\"S\" means STAND - don't pick up any more cards.".to_string(),
                            "\"H\" means HIT - take another card from the deck.".to_string(),
                            "\"D\" means DOUBLE - double your starting bet and pick up only one more card.".to_string(),
                            "Choose your action by clicking the buttons on the right.".to_string(),
                            "The yellow buttons toggle the entire row / column.".to_string(),
                            "Note that you cannot toggle the \"10\" row, becuase A+10 is already 21.".to_string()
                        ],
                        State::Splits => vec![
                            "--- SPLITS ---".to_string(),
                            "If you have two of the same card, you have the option to SPLIT them in two, creating two separate hands.".to_string(),
                            "The rows (numbers 2-A) represent the card value.".to_string(),
                            "The columns (numbers 2-A) represent the dealer's card that is face up at the start of the round.".to_string(),
                            "\"Y\" means YES - split your hand in two.".to_string(),
                            "\"N\" means NO - don't split your hand in two.".to_string(),
                            "Choose your action by clicking the buttons on the right.".to_string(),
                            "The yellow buttons toggle the entire row / column.".to_string(),
                        ],
                        _ => vec!["An error has occured :(".to_string()]
                    },/>
                </div>
                <div class=("chooser-matrix", "container"),>
                    <span />
                    { for (0..10).map(|j| html! {
                        <ActionButton: state=action_button::State::T(match j {
                            9 => 99,
                            _ => j + 2
                        }), onsignal=self.link.callback(move |_| Msg::ToggleColumn(j)), active=true,/>
                    })}
                    { for (0..10).map(|i|
                        {
                            html! {
                                <>
                                <ActionButton: state=action_button::State::T(match self.state {
                                    State::Hard => i + 9,
                                    _ => match i {
                                        9 => 99,
                                        _ => i + 2},
                                }), onsignal=self.link.callback(move |_| Msg::ToggleRow(i)), active=(i!=8 || self.state != State::Soft),/>
                                { for (0..10).map(|j| html! {
                                    <ActionButton: state=convert_state(
                                        match self.state {
                                            State::Hard => self.hard_array[[i,j]],
                                            State::Soft => self.soft_array[[i,j]],
                                            State::Splits => self.splits_array[[i,j]],
                                            State::None => panic!("Shouldn't be here"),
                                        }
                                    ), onsignal=self.link.callback(move |_| Msg::ToggleCell(i, j)), active=(i!=8 || self.state != State::Soft),/>
                                })}
                                </>
                            }
                        }
                    )}
                </div>
            </div>
        }
    }
}
