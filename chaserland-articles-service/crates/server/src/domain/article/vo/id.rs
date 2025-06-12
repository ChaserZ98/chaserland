use super::Identifier;
use serde::{Deserialize, Serialize};

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

impl std::fmt::Display for Id {
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

#[cfg(test)]
mod tests {
    use super::{Id, Identifier};

    #[test]
    fn article_id_case_new() {
        let id = Id::new(1);

        assert_eq!(id.value(), 1);
        assert_eq!(id.to_string(), "1".to_string());
    }

    #[test]
    #[should_panic(expected = "id must be greater than 0")]
    fn article_id_case_new_panic() {
        let _ = Id::new(-1);
    }

    #[test]
    fn article_id_case_try_from_i32() {
        let res = Id::try_from(1);
        let target = Id(1);

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res, target);

        let res = Id::try_from(-1);

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert_eq!(res, "id must be greater than 0");
    }

    #[test]
    fn article_id_case_as_identifier() {
        let res = Id::new(1).as_identifier();
        let target = Identifier::Id(Id(1));

        assert_eq!(res, target);
    }
}
