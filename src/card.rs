pub struct Card {
    rank: String,
    suit: String,
    seal: String,
    edition: String,
    enhancement: String
}

fn new_card(rank: String, suit: String, seal: String, edition: String, enhancement: String) -> Card{
    Card {
        rank,
        suit,
        seal,
        edition,
        enhancement,
    }
}