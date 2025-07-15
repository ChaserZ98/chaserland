use crate::{app::query::dto::TagDTO, domain::tag::entity::Tag};

#[derive(sqlx::FromRow)]
pub struct PgTag {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl TryInto<Tag> for PgTag {
    type Error = String;

    fn try_into(self) -> Result<Tag, Self::Error> {
        let id = self.id.try_into()?;
        let name = self.name.try_into()?;

        let tag = Tag::new(id, name);

        Ok(tag)
    }
}

impl Into<TagDTO> for PgTag {
    fn into(self) -> TagDTO {
        TagDTO {
            id: self.id,
            name: self.name,
            slug: self.slug,
        }
    }
}

#[cfg(test)]
mod test {
    use super::PgTag;
    use crate::{app::query::dto::TagDTO, domain::tag::entity::Tag};

    #[test]
    fn pg_tag_try_into_tag() {
        let pg_tag = PgTag {
            id: 1,
            name: "name".to_string(),
            slug: "name".to_string(),
        };
        let target = Tag::new(1.try_into().unwrap(), "name".try_into().unwrap());

        let tag: Tag = pg_tag.try_into().unwrap();

        assert_eq!(tag, target);
    }

    #[test]
    fn pg_tag_into_tag_dto() {
        let pg_tag = PgTag {
            id: 1,
            name: "name".to_string(),
            slug: "name".to_string(),
        };
        let target = TagDTO {
            id: 1,
            name: "name".to_string(),
            slug: "name".to_string(),
        };

        let tag: TagDTO = pg_tag.into();

        assert_eq!(tag, target);
    }
}
