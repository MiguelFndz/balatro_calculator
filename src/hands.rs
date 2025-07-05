pub enum Hand {
    HighCard,
    Pair,
    TwoPair,
    ThreeOAK,
    Straight,
    Flush,
    FullHouse,
    FourOAK,
    StraightFlush,
    FiveOAK,
    FlushHouse,
    FlushFive
}

impl Hand {
    pub fn chips_and_mult(&self, level: u32) -> (u32, f32) {
        match self {
            Hand::HighCard => {
                let chips:u32 = 5 + (10 * level);
                let mult: f32 = (1 + level) as f32;
                (chips, mult)
            },
            Hand::Pair => {
                let chips: u32 = 10 + (15 * level);
                let mult: f32 = (2 + level) as f32;
                (chips, mult)
            },
            Hand::TwoPair => {
                let chips: u32 = 20 + (20 * level);
                let mult: f32 = (2 + level) as f32;
                (chips, mult)
            },
            Hand::ThreeOAK => {
                let chips: u32 = 30 + (20 * level);
                let mult: f32 = (3 + (2 * level)) as f32;
                (chips, mult)
            },
            Hand::Straight => {
                let chips: u32 = 30 + (30 * level);
                let mult: f32 = (4 + (3 * level)) as f32;
                (chips, mult)
            },
            Hand::Flush => {
                let chips: u32 = 35 + (15 * level);
                let mult: f32 = (4 + (2 * level)) as f32;
                (chips, mult)
            },
            Hand::FullHouse => {
                let chips: u32 = 40 + (25 * level);
                let mult: f32 = (4 + (2 * level)) as f32;
                (chips, mult)
            },
            Hand::FourOAK => {
                let chips: u32 = 60 + (30 * level);
                let mult: f32 = (7 + (3 * level)) as f32;
                (chips, mult)
            },
            Hand::StraightFlush => {
                let chips: u32 = 100 + (40 * level);
                let mult: f32 = (8 + (4 * level)) as f32;
                (chips, mult)
            },
            Hand::FiveOAK => {
                let chips: u32 = 120 + (35 * level);
                let mult: f32 = (12 + (3 * level)) as f32;
                (chips, mult)
            },
            Hand::FlushHouse => {
                let chips: u32 = 140 + (40 * level);
                let mult: f32 = (14 + (4 * level)) as f32;
                (chips, mult)
            },
            Hand::FlushFive => {
                let chips: u32 = 160 + (50 * level);
                let mult: f32 = (16 + (3 * level)) as f32;
                (chips, mult)
            },
        }
    }
}