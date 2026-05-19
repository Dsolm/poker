use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use tokio::sync::RwLock;

use crate::domain::blackjack::{
    models::game::{Card, Game, GameOutcome, HitError, HitRequest, StandError, StandRequest},
    ports::BlackjackRepository,
};

type PlayerId = uuid::Uuid;

#[derive(Clone, Default)]
pub struct PlayerGameMap {
    // TODO: Use DashMap instead
    games: Arc<RwLock<HashMap<PlayerId, Game>>>,
}

impl BlackjackRepository for PlayerGameMap {
    async fn hit(&self, req: &HitRequest) -> Result<(Card, Option<GameOutcome>), HitError> {
        let mut games = self.games.write().await;
        let game = games.get_mut(&req.player());
        if let Some(game) = game {
            game.hit()
        } else {
            Err(anyhow!("Access denied").into())
        }
    }

    async fn stand(&self, req: &StandRequest) -> Result<GameOutcome, StandError> {
        let mut games = self.games.write().await;
        let game = games.get_mut(&req.player());
        if let Some(game) = game {
            game.stand()
        } else {
            Err(anyhow!("Access denied").into())
        }
    }
}
