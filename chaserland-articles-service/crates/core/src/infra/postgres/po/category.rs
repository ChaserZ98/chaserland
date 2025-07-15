use crate::{app::query::dto::CategoryDTO, domain::category::entity::Category};

#[derive(sqlx::FromRow)]
pub struct PgCategory {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl TryInto<Category> for PgCategory {
    type Error = String;

    fn try_into(self) -> Result<Category, Self::Error> {
        let id = self.id.try_into()?;
        let name = self.name.try_into()?;
        let category = Category::new(id, name);
        Ok(category)
    }
}

impl Into<CategoryDTO> for PgCategory {
    fn into(self) -> CategoryDTO {
        CategoryDTO {
            id: self.id,
            name: self.name,
            slug: self.slug,
        }
    }
}
