use async_trait::async_trait;
pub use http_api::*;

use crate::api::signup::SignUp;
use crate::error::Result;

mod http_api;
mod signup;

#[async_trait]
pub trait Auth0<P: Passwordless, D: DbConnection> {
    fn get_login_url() -> Result<()>;
    fn get_logout_url() -> Result<()>;
    fn passwordless() -> P;
    fn db_connection() -> D;
    async fn user_info() -> Result<()>;
}

#[async_trait]
pub trait Passwordless {
    async fn start() -> Result<()>;
    async fn verify() -> Result<()>;
}

#[async_trait]
pub trait DbConnection {
    async fn signup(signup: SignUp) -> Result<()>;
    async fn change_password() -> Result<()>;
}
