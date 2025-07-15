use crate::domain::category::{
    entity::Category, repository::CategoryRepositoryError, vo as category,
};
use sqlx::{Database, Transaction};

#[trait_variant::make(CategoryRepository: Send)]
pub trait LocalCategoryRepository: Clone + Sync + 'static {
    type DB: Database;
    async fn create(
        &self,
        command: category::NewCategory,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Category, CategoryRepositoryError>;

    async fn get_one(
        &self,
        identifier: category::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Category, CategoryRepositoryError>;

    // async fn get_many(
    //     &self,
    //     filter: Option<CategoriesFilter>,
    //     pagination: Option<Pagination>,
    // ) -> Result<Vec<Category>, CategoryRepositoryError>;

    async fn delete(
        &self,
        identifier: category::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), CategoryRepositoryError>;
}
