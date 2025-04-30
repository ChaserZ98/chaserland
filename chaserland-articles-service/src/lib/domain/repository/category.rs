use crate::domain::entity::category;
use async_trait::async_trait;
use chaserland_common::pagination::Pagination;

#[async_trait]
pub trait CategoryRepository: Send + Sync + 'static {
    async fn create(
        &self,
        command: category::NewCategory,
    ) -> Result<category::Category, CategoryRepositoryError>;
    async fn get_one(
        &self,
        identifier: category::Identifier,
    ) -> Result<category::Category, CategoryRepositoryError>;
    async fn get_many(
        &self,
        filter: Option<CategoriesFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<category::Category>, CategoryRepositoryError>;
    async fn delete(&self, identifier: category::Identifier)
    -> Result<(), CategoryRepositoryError>;
}

#[derive(Debug)]
pub struct CategoriesFilter {
    category_ids: Vec<category::Id>,
}

impl CategoriesFilter {
    pub fn new(category_ids: Vec<category::Id>) -> Self {
        Self::validate(&category_ids).unwrap();
        Self { category_ids }
    }

    fn validate(category_ids: &Vec<category::Id>) -> Result<(), String> {
        match category_ids.is_empty() {
            true => Err("At least one category id must be specified".to_string()),
            false => Ok(()),
        }
    }

    pub fn category_ids(&self) -> &Vec<category::Id> {
        &self.category_ids
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CategoryRepositoryError {
    #[error("Category with identifier {0} not found")]
    CategoryNotFound(category::Identifier),
    #[error("Series {} with slug {} already exists", .0.name, .0.slug())]
    DuplicateCategorySlug(category::NewCategory),
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl CategoryRepositoryError {
    pub fn is_category_not_found(&self) -> bool {
        matches!(self, Self::CategoryNotFound(_))
    }

    pub fn is_duplicate_category_slug(&self) -> bool {
        matches!(self, Self::DuplicateCategorySlug(..))
    }

    pub fn is_transaction(&self) -> bool {
        matches!(self, Self::Transaction(_))
    }

    pub fn is_do_conversion(&self) -> bool {
        matches!(self, Self::DOConversion(_))
    }
}
