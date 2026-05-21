use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use tokio::sync::RwLock;

use crate::domain::{
    auth::{
        models::authorized_user::{CreateUserRequest, User},
        ports::UserRepository,
    },
    blackjack::{
        models::game::{Card, Game, GameOutcome, HitError, HitRequest, StandError, StandRequest},
        ports::BlackjackRepository,
    },
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

use anyhow::bail;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Username `{0}` not found")]
pub(crate) struct KeyError(String);

#[derive(Clone)]
pub struct MemoryUserRepository {
    pub(crate) users: Arc<RwLock<Vec<User>>>, // (name, Data)
}

#[derive(Error, Debug)]
#[error("Could not hash password.")]
pub struct PasswordHashError;

impl UserRepository for MemoryUserRepository {
    async fn find_by_username(&self, username: &str) -> anyhow::Result<User> {
        let users = self.users.read().await;
        match users.iter().find(|x| x.username.0 == username) {
            None => Err(KeyError(username.to_string()).into()),
            Some(x) => Ok(x.clone()),
        }
    }

    async fn create_user(
        &self,
        CreateUserRequest {
            id,
            password_hash,
            username,
            email,
        }: CreateUserRequest,
    ) -> anyhow::Result<User> {
        let mut users = self.users.write().await;
        let user = User {
            id,
            password_hash,
            username,
            email,
        };
        users.push(user.clone());
        Ok(user)
    }

    async fn find_by_id(&self, id: &str) -> anyhow::Result<User> {
        let users = self.users.read().await;
        match users.iter().find(|x| x.id == id) {
            Some(x) => Ok(x.clone()),
            None => bail!("User with id `{}` not found.", id),
        }
    }
}
