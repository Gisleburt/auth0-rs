use async_trait::async_trait;

use crate::api::{Auth0, DbConnection, Passwordless};
use crate::error::Result;

pub struct HttpAuth0 {
    client_id: String,
}

impl HttpAuth0 {
    pub fn new(client_id: String) -> HttpAuth0 {
        HttpAuth0 { client_id }
    }
}

#[async_trait]
impl Auth0<HttpPasswordless, HttpDbConnection> for HttpAuth0 {
    fn get_login_url() -> Result<()> {
        Ok(())
    }

    fn get_logout_url() -> Result<()> {
        Ok(())
    }

    fn passwordless() -> HttpPasswordless {
        HttpPasswordless {}
    }

    fn db_connection() -> HttpDbConnection {
        HttpDbConnection {}
    }

    async fn user_info() -> Result<()> {
        Ok(())
    }
}

pub struct HttpPasswordless;

#[async_trait]
impl Passwordless for HttpPasswordless {
    async fn start() -> Result<()> {
        Ok(())
    }

    async fn verify() -> Result<()> {
        Ok(())
    }
}

pub struct HttpDbConnection;

#[async_trait]
impl DbConnection for HttpDbConnection {
    async fn signup() -> Result<()> {
        Ok(())
    }

    async fn change_password() -> Result<()> {
        Ok(())
    }
}
