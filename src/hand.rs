use crate::card::Card;

struct Hand {
    cards: Vec<Card>
}

impl Hand {
    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }
}