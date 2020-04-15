use std::net::IpAddr;

use super::wrappers::{Session, Sessions};
use heim_common::prelude::*;

#[derive(Debug)]
pub struct User {
    domain: String,
    username: String,
    address: Option<IpAddr>,
}

impl User {
    pub fn from_session(session: Session) -> Result<Option<User>> {
        let info = session.info()?;

        let username = match info.username() {
            None => return Ok(None),
            Some(username) => username,
        };
        let domain = info.domain();

        Ok(Some(User {
            domain,
            username,
            address: session.address()?,
        }))
    }

    pub fn domain(&self) -> &str {
        self.domain.as_str()
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    pub fn address(&self) -> Option<&IpAddr> {
        self.address.as_ref()
    }
}

pub fn users() -> Result<impl Iterator<Item = Result<User>>> {
    let sessions = Sessions::new()?;
    let iter = sessions.filter_map(|session| match User::from_session(session) {
        Ok(Some(user)) => Some(Ok(user)),
        Err(e) => Some(Err(e)),
        _ => None,
    });

    Ok(iter)
}
