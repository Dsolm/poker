use anyhow::bail;

use crate::domain::auth::{
    models::authorized_user::{CreateUserRequest, Token, User, create_token, verify_token},
    ports::{AuthService, UserRepository},
};

#[derive(Clone)]
pub struct Service<T: UserRepository> {
    user_repo: T,
    secret: String,
}

impl<T: UserRepository> AuthService for Service<T> {
    async fn create_user(&self, req: CreateUserRequest) -> anyhow::Result<User> {
        self.user_repo.create_user(req).await
    }

    async fn user_login(&self, username: &str, password: &str) -> anyhow::Result<Token> {
        let user = self.user_repo.find_by_username(username).await?;
        if user.password_hash.password_valid(password) {
            Ok(Token(create_token(user.id, &self.secret)?))
        } else {
            bail!("Invalid password")
        }
    }

    async fn verify_token(&self, token: String) -> anyhow::Result<User> {
        let claims = verify_token(&token, &self.secret)?;
        let user_id = claims.sub;
        self.user_repo.find_by_id(&user_id).await
    }
}
