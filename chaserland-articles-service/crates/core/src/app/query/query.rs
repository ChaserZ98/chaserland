use crate::domain::article::{repository::ArticlesFilter, vo as article};
use crate::domain::category::{repository::CategoriesFilter, vo as category};
use crate::domain::series::{repository::SeriesFilter, vo as series};
use crate::domain::tag::{repository::TagsFilter, vo as tag};
use chaserland_common::pagination::Pagination;

pub struct GetArticleOneQuery {
    pub identifier: article::Identifier,
    pub public_only: bool,
    pub with_content: bool,
}

pub struct GetArticleContentQuery {
    pub identifier: article::Identifier,
    pub public_only: bool,
}

pub struct GetArticleManyQuery {
    pub pagination: Pagination,
    pub public_only: bool,
    pub with_content: bool,
    pub filter: Option<ArticlesFilter>,
}

pub struct GetSeriesOneQuery {
    pub identifier: series::Identifier,
}

pub struct GetSeriesManyQuery {
    pub filter: Option<SeriesFilter>,
    pub pagination: Option<Pagination>,
}

pub struct GetCategoryOneQuery {
    pub identifier: category::Identifier,
}

pub struct GetCategoryManyQuery {
    pub filter: Option<CategoriesFilter>,
    pub pagination: Option<Pagination>,
}

pub struct GetTagOneQuery {
    pub identifier: tag::Identifier,
}

pub struct GetTagManyQuery {
    pub filter: Option<TagsFilter>,
    pub pagination: Option<Pagination>,
}
