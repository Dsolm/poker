/*!
  Module `service` provides the canonical implementation of the [BlackJack] port. All
  blog-domain logic is defined here.
*/

use crate::domain::blackjack::{
    models::game::{Card, GameOutcome, HitError, HitRequest},
    ports::{BlackjackRepository, BlackjackService},
};

use super::models::game::StandError;

/// Canonical implementation of the [BlogService] port, through which the blog domain API is
/// consumed.
#[derive(Debug, Clone)]
pub struct Service<R>
where
    R: BlackjackRepository,
{
    repo: R,
}

impl<R> Service<R>
where
    R: BlackjackRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R> BlackjackService for Service<R>
where
    R: BlackjackRepository,
{
    async fn hit(&self, req: &HitRequest) -> Result<(Card, Option<GameOutcome>), HitError> {
        self.repo.hit(req).await
    }

    async fn stand(
        &self,
        req: &super::models::game::StandRequest,
    ) -> Result<GameOutcome, StandError> {
        self.repo.stand(req).await
    }
}
