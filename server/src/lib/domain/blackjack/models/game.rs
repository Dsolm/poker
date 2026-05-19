use thiserror::Error;

#[derive(Debug, Default)]
pub struct Game {
    dealer: Hand,
    player_hand: Hand,
    player_stood: bool,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GameOutcome {
    PlayerBlackjack,
    PlayerWin,
    PlayerBust,
    DealerBlackjack,
    DealerWin,
    DealerBust,
    Standoff,
}

impl Game {
    pub fn check_winner(&self) -> GameOutcome {
        let player_score = self.player_hand.compute_score();
        let dealer_score = self.dealer.compute_score();

        let player_natural_blackjack = player_score == 21 && self.player_hand.cards.len() == 2;
        let dealer_natural_blackjack = dealer_score == 21 && self.dealer.cards.len() == 2;

        use GameOutcome::*;
        if player_natural_blackjack && dealer_natural_blackjack {
            Standoff
        } else if player_natural_blackjack {
            PlayerBlackjack
        } else if dealer_natural_blackjack {
            DealerBlackjack
        } else if player_score > 21 {
            PlayerBust
        } else if dealer_score > 21 {
            DealerBust
        } else if player_score == dealer_score {
            Standoff
        } else if player_score > dealer_score {
            PlayerWin
        } else if dealer_score > player_score {
            DealerWin
        } else {
            unreachable!();
        }
    }

    pub fn hit(&mut self) -> Result<(Card, Option<GameOutcome>), HitError> {
        if self.player_stood {
            return Err(HitError::AlreadyStood);
        }

        let card = self.player_hand.hit();
        let score = self.player_score();
        if score >= 21 {
            Ok((card, Some(self.check_winner())))
        } else {
            Ok((card, None))
        }
    }

    pub fn stand(&mut self) -> Result<GameOutcome, StandError> {
        if self.player_stood {
            return Err(StandError::AlreadyStood);
        }

        self.player_stood = true;
        self.run_dealer();

        Ok(self.check_winner())
    }

    pub fn player_score(&self) -> u32 {
        self.player_hand.compute_score()
    }

    pub fn dealer_cards(&self) -> &Vec<Card> {
        &self.dealer.cards
    }

    fn run_dealer(&mut self) {
        loop {
            let dealer_score = self.dealer.compute_score();
            if dealer_score >= 17 {
                break;
            }
            self.dealer.hit();
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    face: u8,
    value: u8,
}

impl Card {
    pub fn face(&self) -> u8 {
        self.face
    }
    pub fn value(&self) -> u8 {
        self.value
    }
    pub fn random() -> Card {
        Card {
            face: rand::random_range(0..4),
            value: rand::random_range(1..=13),
        }
    }
}

#[derive(Default, Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    fn hit(&mut self) -> Card {
        let card = Card::random();
        self.cards.push(card);
        card
    }

    fn compute_score(&self) -> u32 {
        let mut score: u32 = 0;
        let mut aces: u32 = 0;

        for card in &self.cards {
            if card.value >= 2 && card.value <= 10 {
                score += card.value as u32;
            } else if card.value == 1 {
                aces += 1;
            } else {
                score += 10;
            }
        }

        for _ in 0..aces {
            if score + 11 <= 21 {
                score += 11;
            } else {
                score -= 11;
                score += 1;
                score += 1;
            }
        }

        score
    }
}

#[derive(Debug)]
pub struct HitRequest {
    player_id: uuid::Uuid,
}

impl HitRequest {
    pub fn new(player: uuid::Uuid) -> Self {
        Self { player_id: player }
    }
    pub fn player(&self) -> uuid::Uuid {
        self.player_id
    }
}

#[derive(Debug)]
pub struct StandRequest {
    player_id: uuid::Uuid,
}

impl StandRequest {
	pub fn new(player: uuid::Uuid) -> Self {
		Self { player_id: player }
	}
    pub fn player(&self) -> uuid::Uuid {
        self.player_id
    }
}

#[derive(Debug, Error)]
pub enum HitError {
    #[error("player has already stood")]
    AlreadyStood,
    #[error("player has already busted")]
    AlreadyBusted,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum StandError {
    #[error("player has already stood")]
    AlreadyStood,
    #[error("player has already busted")]
    AlreadyBusted,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
