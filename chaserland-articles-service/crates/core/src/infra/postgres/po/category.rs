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

#[cfg(test)]
mod tests {
    use super::PgCategory;
    use crate::{app::query::dto::CategoryDTO, domain::category::entity::Category};

    #[test]
    fn pg_category_try_into_category() {
        let pg_category = PgCategory {
            id: 1,
            name: "name".to_string(),
            slug: "name".to_string(),
        };
        let target = Category::new(1.try_into().unwrap(), "name".try_into().unwrap());

        let category: Category = pg_category.try_into().unwrap();

        assert_eq!(category, target);
    }

    #[test]
    fn pg_article_into_category_dto() {
        let pg_category = PgCategory {
            id: 1,
            name: "name".to_string(),
            slug: "name".to_string(),
        };
        let target = CategoryDTO {
            id: 1,
            name: "name".to_string(),
            slug: "name".to_string(),
        };

        let category: CategoryDTO = pg_category.into();

        assert_eq!(category, target);
    }
}
