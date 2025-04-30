use super::error::ArticleServiceError;
use crate::app::dto;
use crate::domain::entity::{article, category, series, tag};
use crate::domain::repository::article::ArticlesFilter;
use crate::domain::repository::{
    article::ArticleRepository, category::CategoryRepository, series::SeriesRepository,
    tag::TagRepository,
};
use chaserland_common::pagination::{Page, PageSize};

pub struct ArticleService<R, S, C, T>
where
    R: ArticleRepository,
    S: SeriesRepository,
    C: CategoryRepository,
    T: TagRepository,
{
    article_repository: R,
    series_repository: S,
    category_repository: C,
    tag_repository: T,
}

impl<R, S, C, T> ArticleService<R, S, C, T>
where
    R: ArticleRepository,
    S: SeriesRepository,
    C: CategoryRepository,
    T: TagRepository,
{
    pub fn new(
        article_repository: R,
        series_repository: S,
        category_repository: C,
        tag_repository: T,
    ) -> Self {
        Self {
            article_repository,
            series_repository,
            category_repository,
            tag_repository,
        }
    }
    pub async fn create_article(
        &self,
        article: article::ArticleCreate,
    ) -> Result<dto::ArticleDTO, ArticleServiceError> {
        let series = match &article.series_id {
            None => None,
            Some(id) => Some(
                self.series_repository
                    .get_one(id.as_identifier())
                    .await
                    .map_err(|why| ArticleServiceError::Repository(why.into()))?,
            ),
        };
        let mut categories = vec![];
        for category_id in &article.category_ids {
            let category = self
                .category_repository
                .get_one(category_id.as_identifier())
                .await
                .map_err(|why| ArticleServiceError::Repository(why.into()))?;
            categories.push(category);
        }

        let mut tags = vec![];
        for tag_id in &article.tag_ids {
            let tag = self
                .tag_repository
                .get_one(tag_id.as_identifier())
                .await
                .map_err(|why| ArticleServiceError::Repository(why.into()))?;
            tags.push(tag);
        }

        let article = self
            .article_repository
            .create(article)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        let mut article_dto_builder = dto::ArticleDTOBuilder::new().with_article(article);
        if let Some(series) = series {
            article_dto_builder = article_dto_builder.with_series(series);
        }
        article_dto_builder = article_dto_builder
            .with_categories(categories)
            .with_tags(tags);

        let article_dto = article_dto_builder
            .try_build()
            .map_err(|why| ArticleServiceError::DTOConversion(why))?;

        Ok(article_dto)
    }
    pub async fn get_article(
        &self,
        identifier: article::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<article::Article, ArticleServiceError> {
        let article = self
            .article_repository
            .get_one(identifier, public_only, with_content)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(article)
    }
    pub async fn get_articles(
        &self,
        page: Page,
        page_size: PageSize,
        public_only: bool,
        with_content: bool,
        filter: Option<ArticlesFilter>,
    ) -> Result<Vec<article::Article>, ArticleServiceError> {
        let articles = self
            .article_repository
            .get_many(page, page_size, public_only, with_content, filter)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(articles)
    }
    pub async fn get_series(
        &self,
        identifier: series::Identifier,
    ) -> Result<series::Series, ArticleServiceError> {
        let series = self
            .series_repository
            .get_one(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(series)
    }
    pub async fn get_category(
        &self,
        identifier: category::Identifier,
    ) -> Result<category::Category, ArticleServiceError> {
        let category = self
            .category_repository
            .get_one(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(category)
    }
    pub async fn get_tag(
        &self,
        identifier: tag::Identifier,
    ) -> Result<tag::Tag, ArticleServiceError> {
        let tag = self
            .tag_repository
            .get_one(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(tag)
    }
    pub async fn publish_article(
        &self,
        identifier: article::Identifier,
    ) -> Result<(), ArticleServiceError> {
        let mut article = self
            .article_repository
            .get_one(identifier, false, false)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        article
            .publish()
            .map_err(|why| ArticleServiceError::Domain(why.into()))?;

        self.article_repository
            .publish(
                article.id,
                article.published_at.clone().unwrap(),
                article.version,
            )
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(())
    }
    pub async fn delete_article(
        &self,
        identifier: article::Identifier,
    ) -> Result<(), ArticleServiceError> {
        self.article_repository
            .delete(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(())
    }
    pub async fn delete_series(
        &self,
        identifier: series::Identifier,
    ) -> Result<(), ArticleServiceError> {
        self.series_repository
            .delete(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(())
    }
    pub async fn delete_category(
        &self,
        identifier: category::Identifier,
    ) -> Result<(), ArticleServiceError> {
        self.category_repository
            .delete(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(())
    }
    pub async fn delete_tag(&self, identifier: tag::Identifier) -> Result<(), ArticleServiceError> {
        self.tag_repository
            .delete(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(())
    }
}
