use crate::card::Card;

struct Hand {
    held_cards: Vec<Card>,
    played_cards: Vec<Card>,
}

impl Hand {
    fn add_to_held(&mut self, card: Card) {
        self.held_cards.push(card);
    }

    fn add_to_play(&mut self, index: u32) {
        self.played_cards.push(self.held_cards.remove(index));
    }
}