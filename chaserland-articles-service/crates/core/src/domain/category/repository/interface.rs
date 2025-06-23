use crate::domain::category::{
    entity::Category,
    repository::{CategoriesFilter, CategoryRepositoryError},
    vo as category,
};
use chaserland_common::pagination::Pagination;

#[trait_variant::make(CategoryRepository: Send)]
pub trait LocalCategoryRepository: Clone + Sync + 'static {
    async fn create(
        &self,
        command: category::NewCategory,
    ) -> Result<Category, CategoryRepositoryError>;

    async fn get_one(
        &self,
        identifier: category::Identifier,
    ) -> Result<Category, CategoryRepositoryError>;

    async fn get_many(
        &self,
        filter: Option<CategoriesFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Category>, CategoryRepositoryError>;

    async fn delete(&self, identifier: category::Identifier)
    -> Result<(), CategoryRepositoryError>;
}
