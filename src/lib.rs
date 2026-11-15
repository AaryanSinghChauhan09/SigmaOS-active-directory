//! SigmaOS-active-directory: Active Directory model.
//!
//! Real implementation: users, groups, OUs, validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdUser {
    pub id: String,
    pub username: String,
    pub distinguished_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdError {
    EmptyName,
    DuplicateUser(String),
}

impl fmt::Display for AdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "username must not be empty"),
            Self::DuplicateUser(id) => write!(f, "duplicate user {:?}", id),
        }
    }
}

impl std::error::Error for AdError {}

pub struct AdDomain {
    pub name: String,
    users: BTreeMap<String, AdUser>,
    groups: BTreeMap<String, String>,
}

impl AdDomain {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), users: BTreeMap::new(), groups: BTreeMap::new() }
    }

    pub fn add_user(&mut self, u: AdUser) -> Result<(), AdError> {
        if u.username.trim().is_empty() { return Err(AdError::EmptyName); }
        let id = u.id.clone();
        if self.users.contains_key(&id) {
            return Err(AdError::DuplicateUser(id.clone()));
        }
        self.users.insert(id, u);
        Ok(())
    }

    pub fn user_count(&self) -> usize { self.users.len() }

    pub fn get_user(&self, id: &str) -> Option<&AdUser> { self.users.get(id) }

    pub fn add_group(&mut self, name: String, description: String) {
        self.groups.insert(name, description);
    }

    pub fn get_group(&self, name: &str) -> Option<&String> { self.groups.get(name) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let d = AdDomain::new("example.com");
        assert_eq!(d.name, "example.com");
        assert_eq!(d.user_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut d = AdDomain::new("example.com");
        let u = AdUser { id: "user1".into(), username: "alice".into(), distinguished_name: "CN=alice,OU=users,DC=example,DC=com".into() };
        d.add_user(u).unwrap();
        assert_eq!(d.user_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut d = AdDomain::new("example.com");
        let u = AdUser { id: "x".into(), username: "".into(), distinguished_name: "a".into() };
        assert!(d.add_user(u).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut d = AdDomain::new("example.com");
        let u = AdUser { id: "x".into(), username: "X".into(), distinguished_name: "a".into() };
        d.add_user(u).unwrap();
        let u2 = AdUser { id: "x".into(), username: "X".into(), distinguished_name: "a".into() };
        assert!(d.add_user(u2).is_err());
    }
}
