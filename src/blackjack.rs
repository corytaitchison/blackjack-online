use ndarray::Array2;
use rand::{seq::SliceRandom, thread_rng};

const SHUFFLE_SIZE: usize = 260;
const RESHUFFLE: usize = 18;
const NUM_DECKS: usize = 6;
const BUST_KWD: &str = &"bust";
const STARTING_MONEY: usize = 1_000_000;
const NUM_LOOPS: usize = 50_000;
const BET_UNIT: usize = 1;

// --- CARDS ---

#[derive(Copy, Clone)]
enum Card {
    Def(u8),
    Maybe(u8, u8),
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Card::Def(n) => format!("{}", n).fmt(f),
            Card::Maybe(x, y) => format!("{} or {}", x, y).fmt(f),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Card::Def(n) => format!("{}", n).fmt(f),
            Card::Maybe(x, y) => format!("{} or {}", x, y).fmt(f),
        }
    }
}

impl std::ops::Add for Card {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Def(a), Self::Def(b)) => Self::Def(a + b),
            (Self::Def(a), Self::Maybe(b, c)) | (Self::Maybe(b, c), Self::Def(a)) => {
                if a + c > 21 {
                    Self::Def(a + b)
                } else {
                    Self::Maybe(a + b, a + c)
                }
            }
            (Self::Maybe(a, b), _) => Self::Maybe(a + 1, b + 1),
        }
    }
}

impl std::ops::AddAssign for Card {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Def(a), Self::Def(b)) => a == b,
            (Self::Maybe(a, b), Self::Maybe(c, d)) => (a == c) && (b == d),
            _ => false,
        }
    }
}

macro_rules! real_sum {
    ( $e:expr ) => {
        match $e.sum {
            Card::Def(n) | Card::Maybe(_, n) => n,
        }
    };
}

macro_rules! to_index {
    ( $e:expr ) => {
        match $e {
            Card::Def(n) | Card::Maybe(_, n) => (n - 2) as usize,
        }
    };
}

#[derive(Clone)]
struct Cards {
    cards: Vec<Card>,
}

impl Cards {
    fn sum(&self) -> Card {
        let mut total = Card::Def(0);
        for card in self.cards.iter() {
            total += *card;
        }
        total
    }
}

// --- DECK ---

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
    drawables: Vec<Card>,
    counter: isize,
}

impl Deck {
    fn new() -> Self {
        let mut deck: Vec<Card> = (0..52 * NUM_DECKS).map(|_| Card::Def(1)).collect();
        const VALS: [Card; 13] = [
            Card::Maybe(1, 11),
            Card::Def(2),
            Card::Def(3),
            Card::Def(4),
            Card::Def(5),
            Card::Def(6),
            Card::Def(7),
            Card::Def(8),
            Card::Def(9),
            Card::Def(10),
            Card::Def(10),
            Card::Def(10),
            Card::Def(10),
        ];
        for i in 0..52 * NUM_DECKS {
            deck[i] = VALS[i % 13];
        }
        Deck {
            cards: deck,
            drawables: Vec::new(),
            counter: 0,
        }
    }

    fn shuffle(&mut self) {
        self.drawables = self
            .cards
            .choose_multiple(&mut thread_rng(), SHUFFLE_SIZE)
            .map(|&c| c)
            .collect();
        self.counter = 0;
    }

    fn draw(&mut self) -> Card {
        match self.drawables.pop() {
            Some(n) => {
                self.counter += match n {
                    Card::Maybe(_, _) | Card::Def(10) => 1,
                    Card::Def(d) if d < 7 => -1,
                    _ => 0,
                };
                n
            }
            None => panic!("Not enough cards!"),
        }
    }
}

// --- HAND ---

struct Hand {
    cards: Cards,
    sum: Card,
    busted: bool,
}

impl Hand {
    fn new(deck: &mut Deck) -> Self {
        let cards: Cards = Cards {
            cards: (0..2).map(|_| deck.draw()).collect(),
        };
        Hand {
            sum: cards.sum(),
            busted: false,
            cards,
        }
    }

    fn hit(&mut self, deck: &mut Deck) {
        let card = deck.draw();
        self.cards.cards.push(card);
        self.sum += card;
    }
}

// --- MONEY ---

struct Wallet {
    balance: usize,
    bet: usize,
}

impl Wallet {
    fn new() -> Self {
        Wallet {
            balance: STARTING_MONEY,
            bet: 0,
        }
    }

    fn place_bet(&mut self, amount: usize) -> Result<(), ()> {
        if self.balance >= amount {
            self.balance -= amount;
            self.bet += amount;
            Ok(())
        } else {
            Err(())
        }
    }

    fn double(&mut self) -> Result<(), ()> {
        self.place_bet(self.bet)
    }

    fn pay_out(&mut self, multiplier: usize) {
        self.balance += multiplier * self.bet;
        self.bet = 0;
    }

    fn lose(&mut self) {
        self.bet = 0;
    }
}

// --- PROGRAM ---

fn choice(input: &str, deck: &mut Deck, hand: &mut Hand, wallet: &mut Wallet) -> bool {
    match input {
        "s" => return false,
        "h" => hand.hit(deck),
        "d" => {
            match wallet.double() {
                Ok(_) => {
                    hand.hit(deck);
                    // hand.show();
                    return false;
                }
                Err(_) => {
                    println!("Balance too low (${})", wallet.balance);
                }
            };
        }
        BUST_KWD => return false,
        _ => (),
    }
    true
    // True means keep playing the round
}

#[allow(unused_assignments)]
#[allow(unused_macros)]
pub fn play(
    rule_hard: &Array2<&'static str>,
    rule_soft: &Array2<&'static str>,
    rule_split: &Array2<&'static str>,
) -> usize {
    let mut wallet = Wallet::new();

    let mut deck = Deck::new();

    let mut running_balance: Vec<usize> = Vec::new();

    'play: for _ in 0..NUM_LOOPS {
        deck.shuffle();

        'main: loop {
            let remaining_cards = deck.drawables.len();
            if remaining_cards < RESHUFFLE {
                break 'main;
            }

            if let Err(_) = wallet.place_bet(BET_UNIT) {
                // println!("Balance too low (${})", wallet.balance);
                running_balance.push(wallet.balance);
                break 'play;
            }

            let mut hand = Hand::new(&mut deck);

            if hand.sum == Card::Maybe(11, 21) {
                wallet.pay_out(3);
                continue 'main;
            }

            let mut dealer = Hand::new(&mut deck);

            // --- Inputs ---

            let mut split = false;
            let mut bet = 0usize;
            let mut hand2 = Hand {
                cards: Cards { cards: vec![] },
                busted: false,
                sum: Card::Def(0),
            };

            macro_rules! player_input {
                ( $e:expr ) => {{
                    $e.show();
                    if match $e.sum {
                        Card::Def(n) => n > 21,
                        _ => false,
                    } {
                        println!("Busted!");
                        $e.busted = true;
                        BUST_KWD
                    } else {
                        match &stdin().lock().lines().next().unwrap().unwrap()[..] {
                            "s" => "s",
                            "h" => "h",
                            "d" => "d",
                            "sp" => {
                                if $e.cards.cards[0] == $e.cards.cards[1]
                                    && wallet.balance >= wallet.bet
                                    && bet == 0
                                {
                                    split = true;
                                    $e.cards.cards.remove(1);
                                    $e.sum = $e.cards.cards[0];
                                    "h"
                                } else {
                                    "tortoise"
                                }
                            }
                            "q" => break 'play,
                            _ => "tortoise",
                        }
                    }
                }};
            }

            macro_rules! dealer_input {
                () => {{
                    if real_sum!(dealer) < 17 {
                        "h"
                    } else {
                        if match dealer.sum {
                            Card::Def(n) => n > 21,
                            _ => false,
                        } {
                            dealer.busted = true;
                            BUST_KWD
                        } else {
                            "s"
                        }
                    }
                }};
            }

            // let rule_split = arr2(&[
            //     ["y", "y", "y", "y", "y", "y", "n", "n", "n", "n"], // 2,2
            //     ["y", "y", "y", "y", "y", "y", "n", "n", "n", "n"], // 3,3
            //     ["n", "n", "n", "y", "y", "n", "n", "n", "n", "n"],
            //     ["n", "n", "n", "n", "n", "n", "n", "n", "n", "n"],
            //     ["y", "y", "y", "y", "y", "n", "n", "n", "n", "n"],
            //     ["y", "y", "y", "y", "y", "y", "n", "n", "n", "n"],
            //     ["y", "y", "y", "y", "y", "y", "y", "y", "y", "y"],
            //     ["y", "y", "y", "y", "y", "n", "y", "y", "n", "n"],
            //     ["n", "n", "n", "n", "n", "n", "n", "n", "n", "n"], // 10, 10
            //     ["y", "y", "y", "y", "y", "y", "y", "y", "y", "y"], // A,A
            // ]);

            // let rule_soft = arr2(&[
            //     ["h", "h", "h", "d", "d", "h", "h", "h", "h", "h"], //A,2
            //     ["h", "h", "h", "d", "d", "h", "h", "h", "h", "h"], //A,3
            //     ["h", "h", "d", "d", "d", "h", "h", "h", "h", "h"], //A,4
            //     ["h", "h", "d", "d", "d", "h", "h", "h", "h", "h"],
            //     ["h", "d", "d", "d", "d", "h", "h", "h", "h", "h"],
            //     ["d", "d", "d", "d", "d", "s", "s", "h", "h", "h"],
            //     ["s", "s", "s", "s", "d", "s", "s", "s", "s", "s"],
            //     ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"],
            //     ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"], // A,10
            //     ["h", "h", "h", "d", "d", "h", "h", "h", "h", "h"], //A,A
            // ]);

            // let rule_hard = arr2(&[
            //     ["s", "d", "d", "d", "d", "s", "s", "s", "s", "s"], // 9
            //     ["d", "d", "d", "d", "d", "d", "d", "d", "s", "s"], // 10
            //     ["d", "d", "d", "d", "d", "d", "d", "d", "d", "d"], // 11
            //     ["h", "h", "s", "s", "s", "h", "h", "h", "h", "h"],
            //     ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"],
            //     ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"],
            //     ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"],
            //     ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"], //16
            //     ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"], //17
            //     ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"], //18
            // ]);

            // BACKUP
            // hard_array: arr2(&[
            //     ["s", "d", "d", "d", "d", "s", "s", "s", "s", "s"], // 9
            //     ["d", "d", "d", "d", "d", "d", "d", "d", "s", "s"], // 10
            //     ["d", "d", "d", "d", "d", "d", "d", "d", "d", "d"], // 11
            //     ["h", "h", "s", "s", "s", "h", "h", "h", "h", "h"],
            //     ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"],
            //     ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"],
            //     ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"],
            //     ["s", "s", "s", "s", "s", "h", "h", "h", "h", "h"], //16
            //     ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"], //17
            //     ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"], //18
            // ]),
            // soft_array: arr2(&[
            //     ["h", "h", "h", "d", "d", "h", "h", "h", "h", "h"], //A,2
            //     ["h", "h", "h", "d", "d", "h", "h", "h", "h", "h"], //A,3
            //     ["h", "h", "d", "d", "d", "h", "h", "h", "h", "h"], //A,4
            //     ["h", "h", "d", "d", "d", "h", "h", "h", "h", "h"],
            //     ["h", "d", "d", "d", "d", "h", "h", "h", "h", "h"],
            //     ["d", "d", "d", "d", "d", "s", "s", "h", "h", "h"],
            //     ["s", "s", "s", "s", "d", "s", "s", "s", "s", "s"],
            //     ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"],
            //     ["s", "s", "s", "s", "s", "s", "s", "s", "s", "s"], //A,10
            //     ["h", "h", "h", "d", "d", "h", "h", "h", "h", "h"], //A,A
            // ]),
            // splits_array: arr2(&[
            //     ["y", "y", "y", "y", "y", "y", "n", "n", "n", "n"], // 2,2
            //     ["y", "y", "y", "y", "y", "y", "n", "n", "n", "n"], // 3,3
            //     ["n", "n", "n", "y", "y", "n", "n", "n", "n", "n"],
            //     ["n", "n", "n", "n", "n", "n", "n", "n", "n", "n"],
            //     ["y", "y", "y", "y", "y", "n", "n", "n", "n", "n"],
            //     ["y", "y", "y", "y", "y", "y", "n", "n", "n", "n"],
            //     ["y", "y", "y", "y", "y", "y", "y", "y", "y", "y"],
            //     ["y", "y", "y", "y", "y", "n", "y", "y", "n", "n"],
            //     ["n", "n", "n", "n", "n", "n", "n", "n", "n", "n"], // 10, 10
            //     ["y", "y", "y", "y", "y", "y", "y", "y", "y", "y"], // A,A
            // ]),

            macro_rules! basic_input {
                ( $e:expr ) => {{
                    let dealer_up = dealer.cards.cards[0];

                    if match $e.sum {
                        Card::Def(n) => n > 21,
                        _ => false,
                    } {
                        hand.busted = true;
                        BUST_KWD
                    } else {
                        if $e.cards.cards[0] == $e.cards.cards[1]
                        && bet == 0
                        && wallet.balance >= wallet.bet
                        && rule_split[[to_index!($e.cards.cards[0]), to_index!(dealer_up)]] == "y"
                        {
                        split = true;
                        $e.cards.cards.remove(1);
                        $e.sum = $e.cards.cards[0];
                        "h"
                        } else {
                            match $e.sum {
                                Card::Maybe(_, 21) => "s",
                                Card::Maybe(_, n) => {
                                    rule_soft[[((match n {
                                        12 => 9,
                                        _ => n-13
                                    })) as usize, to_index!(dealer_up)]]
                                }
                                Card::Def(n) => {
                                    if n <= 8 { "h" } else {
                                        if n >= 19 {"s"} else {
                                            rule_hard[[(n-9) as usize, to_index!(dealer_up)]]
                                        }
                                    }
                                },
                            }
                        }
                    }
                }};
            }

            macro_rules! no_input {
                () => {
                    if real_sum!(hand) < 17 {
                        "h"
                    } else {
                        if match hand.sum {
                            Card::Def(n) => n > 21,
                            _ => false,
                        } {
                            hand.busted = true;
                            BUST_KWD
                        } else {
                            "s"
                        }
                    }
                };
            }

            // --- Success Validation ---

            macro_rules! win_lose {
                ( $e:expr ) => {
                    let hand_final = real_sum!($e);
                    let dealer_final = real_sum!(dealer);
                    if !$e.busted && (hand_final >= dealer_final || dealer.busted) {
                        if hand_final == dealer_final {
                            // println!("Push!");
                            wallet.pay_out(1);
                        } else {
                            // println!("You Win!");
                            wallet.pay_out(2);
                        }
                    } else {
                        // println!("You Lose!");
                        wallet.lose();
                    }
                };
            }

            // --- Play ---
            while choice(dealer_input!(), &mut deck, &mut dealer, &mut wallet) {}

            while choice(basic_input!(hand), &mut deck, &mut hand, &mut wallet) {
                if split {
                    bet = wallet.bet;
                    let val = hand.cards.cards[0] + Card::Def(0);
                    hand2 = Hand {
                        cards: Cards { cards: vec![val] },
                        busted: false,
                        sum: val,
                    };
                    hand2.hit(&mut deck);

                    // println!("--- HAND 1 ---");
                    while choice(basic_input!(hand2), &mut deck, &mut hand2, &mut wallet) {}

                    // println!("--- HAND 2 ---");
                    split = false;
                }
            }

            if bet != 0 {
                win_lose!(hand2);
                wallet.place_bet(bet).unwrap();
            }

            win_lose!(hand);
            running_balance.push(wallet.balance);
        }

        // println!("Reshuffling cards...");
    }

    wallet.balance
}
