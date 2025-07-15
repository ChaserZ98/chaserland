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
