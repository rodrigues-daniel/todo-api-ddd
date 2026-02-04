use crate::domain::errors::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

#[derive(Validate)]
struct EmailValidator<'a> {
    #[validate(email)]
    email: &'a str,
}

impl Email {
    pub fn new(email: impl Into<String>) -> DomainResult<Self> {
        let email = email.into();

        let validator = EmailValidator { email: &email };

        if validator.validate().is_err() {
            return Err(DomainError::InvalidEmail(email));
        }

        Ok(Self(email.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
