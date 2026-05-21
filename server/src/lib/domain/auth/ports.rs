// use crate::domain::auth::models::authorized_user::AuthenticatedUser;
use crate::domain::auth::models::authorized_user::{CreateUserRequest, Token, User};
use anyhow::Result;

pub trait AuthService: Clone + Send + Sync + 'static {
    fn create_user(&self, req: CreateUserRequest) -> impl Future<Output = Result<User>> + Send;
    fn user_login(
        &self,
        username: &str,
        password: &str,
    ) -> impl Future<Output = Result<Token>> + Send;
    fn verify_token(&self, token: String) -> impl Future<Output = Result<User>> + Send;
}

pub trait UserRepository: Clone + Send + Sync + 'static {
    fn find_by_username(&self, username: &str) -> impl Future<Output = Result<User>> + Send;
    fn create_user(&self, req: CreateUserRequest) -> impl Future<Output = Result<User>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = Result<User>> + Send;
}
