use std::collections::HashMap;

use crate::api::{Auth0, DbConnection, Passwordless};

pub struct SignUp {
    /// The client_id of your client.
    pub client_id: String,
    /// The user's email address.
    pub email: String,
    /// The user's desired password.
    pub password: String,
    /// The name of the database configured to your client.
    pub connection: String,
    /// The user's username. Only valid if the connection requires a username.
    pub username: Option<String>,
    /// The user's given name(s).
    pub given_name: Option<String>,
    /// The user's family name(s).
    pub family_name: Option<String>,
    /// The user's full name.
    pub name: Option<String>,
    /// The user's nickname.
    pub nickname: Option<String>,
    /// A URI pointing to the user's picture.
    pub picture: Option<String>,
    /// The user metadata to be associated with the user. If set, the field must be an object containing no more than ten properties. Property names can have a maximum of 100 characters, and property values must be strings of no more than 500 characters.
    pub user_metadata: HashMap<String, String>,
}
