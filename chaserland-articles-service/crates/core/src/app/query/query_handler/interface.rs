use super::error;
use crate::{
    app::query::dto,
    domain::{
        article::{repository::ArticlesFilter, vo as article},
        category::{repository::CategoriesFilter, vo as category},
        series::{repository::SeriesFilter, vo as series},
        tag::{repository::TagsFilter, vo as tag},
    },
};
use chaserland_common::pagination::Pagination;

#[trait_variant::make(ArticleQueryHandler: Send)]
pub trait LocalArticleQueryHandler: Clone + Sync + 'static {
    async fn get_one(
        &self,
        identifier: article::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<dto::ArticleDTO, error::ArticleQueryHandlerError>;

    async fn get_many(
        &self,
        pagination: Pagination,
        public_only: bool,
        with_content: bool,
        filter: Option<ArticlesFilter>,
    ) -> Result<Vec<dto::ArticleDTO>, error::ArticleQueryHandlerError>;
}

#[trait_variant::make(SeriesQueryHandler: Send)]
pub trait LocalSeriesQueryHandler: Clone + Sync + 'static {
    async fn get_one(
        &self,
        identifier: series::Identifier,
    ) -> Result<dto::SeriesDTO, error::SeriesQueryHandlerError>;

    async fn get_many(
        &self,
        filter: Option<SeriesFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<dto::SeriesDTO>, error::SeriesQueryHandlerError>;
}

#[trait_variant::make(CategoryQueryHandler: Send)]
pub trait LocalCategoryQueryHandler: Clone + Sync + 'static {
    async fn get_one(
        &self,
        identifier: category::Identifier,
    ) -> Result<dto::CategoryDTO, error::CategoryQueryHandlerError>;

    async fn get_many(
        &self,
        filter: Option<CategoriesFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<dto::CategoryDTO>, error::CategoryQueryHandlerError>;
}

#[trait_variant::make(TagQueryHandler: Send)]
pub trait LocalTagQueryHandler: Clone + Sync + 'static {
    async fn get_one(
        &self,
        identifier: tag::Identifier,
    ) -> Result<dto::TagDTO, error::TagQueryHandlerError>;

    async fn get_many(
        &self,
        filter: Option<TagsFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<dto::TagDTO>, error::TagQueryHandlerError>;
}
