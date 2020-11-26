use super::error::Error;
use super::messages::Dict;

pub trait AuthMethod {
    fn auth_method(&self) -> &str;

    fn challenge(&mut self, auth_id: &str, extra: &Dict) -> Result<(String, Dict), Error>;
}

pub mod wampcra;
