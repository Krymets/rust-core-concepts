use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // Lost of 'suits' - 'hearts', 'spades'
        // let suits = vec!["Hearts", "Spades", "Diamonds"]; VECTOR
        let suits = ["Hearts", "Spades", "Diamonds"];
        // Lost of 'values' - 'ace', 'two', 'three'
        // let values = vec!["Ace", "Two", "Three"]; VECTOR
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        // Double nested for loop
        for suit in suits {
            for value in values {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }
        // let deck = Deck { cards };
        // return deck;

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let cards = deck.deal(7);
    println!("Heres your deck: {:#?}", deck);
}
