use super::Identifier;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Id(i32);

impl Id {
    pub fn new(id: i32) -> Self {
        Self::validate(id).unwrap();
        Self(id)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
    pub fn validate(id: i32) -> Result<(), String> {
        match id > 0 {
            true => Ok(()),
            false => Err("id must be greater than 0".to_string()),
        }
    }
    pub fn as_identifier(&self) -> Identifier {
        self.clone().into()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<i32> for Id {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
