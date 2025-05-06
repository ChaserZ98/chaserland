use super::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Id(i32);

impl Id {
    pub fn new(id: i32) -> Self {
        Self::validate(id).unwrap();
        Self(id)
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn as_identifier(&self) -> Identifier {
        self.clone().into()
    }

    fn validate(id: i32) -> Result<(), String> {
        match id > 0 {
            true => Ok(()),
            false => Err("id must be greater than 0".to_string()),
        }
    }
}

impl TryFrom<i32> for Id {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Id, Identifier};

    #[test]
    fn id_case_new() {
        let id = Id::new(1);
        assert_eq!(id.value(), 1);
    }

    #[test]
    #[should_panic(expected = "id must be greater than 0")]
    fn id_case_new_panic() {
        Id::new(0);
    }

    #[test]
    fn id_case_to_string() {
        let id = Id::new(1);
        assert_eq!(id.to_string(), "1".to_string());
    }

    #[test]
    fn id_case_try_from_i32() {
        let res = Id::try_from(1);

        assert!(res.is_ok());

        let id = res.unwrap();
        assert_eq!(id.value(), 1);

        let res = Id::try_from(-1);

        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, "id must be greater than 0");
    }

    #[test]
    fn id_case_as_identifier() {
        let id = Id::new(1);
        assert_eq!(id.as_identifier(), Identifier::Id(id));
    }
}
