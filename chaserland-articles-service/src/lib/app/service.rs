use super::error::ArticleServiceError;
use super::{command, query};
use crate::app::dto;
use crate::domain::entity::{article, category, series, tag};
use crate::domain::repository::category::CategoriesFilter;
use crate::domain::repository::tag::TagsFilter;
use crate::domain::repository::{
    article::ArticleRepository, category::CategoryRepository, series::SeriesRepository,
    tag::TagRepository,
};
use chaserland_common::app_service_retry::{RetryAsyncFn, RetryErrorPolicy};

pub struct MaxRetryAsyncHandler<E>
where
    E: std::error::Error,
{
    pub max_retry: u32,
    pub should_retry: fn(&E) -> bool,
}

impl<E> RetryErrorPolicy<E> for MaxRetryAsyncHandler<E>
where
    E: std::error::Error,
{
    fn set_policy(&mut self, should_retry: fn(&E) -> bool) {
        self.should_retry = should_retry;
    }
}

impl<T, E, F, Fut> RetryAsyncFn<T, E, F, Fut> for MaxRetryAsyncHandler<E>
where
    E: std::error::Error,
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    async fn run(&self, f: F) -> Result<T, E> {
        let mut result = f().await;
        for _ in 0..self.max_retry {
            if result.is_ok() || !(self.should_retry)(result.as_ref().err().unwrap()) {
                break;
            }
            result = f().await;
        }
        result
    }
}

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
        command: command::CreateArticleCommand,
    ) -> Result<dto::ArticleDTO, ArticleServiceError> {
        let new_article = article::NewArticle::new(
            command.title,
            command.description,
            command.content,
            command.series_id,
            command.category_ids,
            command.tag_ids,
        );
        let series = match &new_article.series_id {
            None => None,
            Some(id) => Some(
                self.series_repository
                    .get_one(id.as_identifier())
                    .await
                    .map_err(|why| ArticleServiceError::Repository(why.into()))?,
            ),
        };
        let mut categories = vec![];
        for category_id in &new_article.category_ids {
            let category = self
                .category_repository
                .get_one(category_id.as_identifier())
                .await
                .map_err(|why| ArticleServiceError::Repository(why.into()))?;
            categories.push(category);
        }

        let mut tags = vec![];
        for tag_id in &new_article.tag_ids {
            let tag = self
                .tag_repository
                .get_one(tag_id.as_identifier())
                .await
                .map_err(|why| ArticleServiceError::Repository(why.into()))?;
            tags.push(tag);
        }

        let article = self
            .article_repository
            .create(new_article)
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
            .map_err(|why| ArticleServiceError::DTOConversion(why.into()))?;

        Ok(article_dto)
    }

    pub async fn create_series(
        &self,
        command: command::CreateSeriesCommand,
    ) -> Result<dto::SeriesDTO, ArticleServiceError> {
        let new_series = series::NewSeries::new(command.name);
        let series = self
            .series_repository
            .create(new_series)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        let series_dto = series.into();
        Ok(series_dto)
    }

    pub async fn create_category(
        &self,
        command: command::CreateCategoryCommand,
    ) -> Result<dto::CategoryDTO, ArticleServiceError> {
        let new_category = category::NewCategory::new(command.name);
        let category = self
            .category_repository
            .create(new_category)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        let category_dto = category.into();
        Ok(category_dto)
    }

    pub async fn create_tag(
        &self,
        command: command::CreateTagCommand,
    ) -> Result<dto::TagDTO, ArticleServiceError> {
        let new_tag = tag::NewTag::new(command.name);
        let tag = self
            .tag_repository
            .create(new_tag)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        let tag_dto = tag.into();
        Ok(tag_dto)
    }

    pub async fn get_article_one(
        &self,
        query: query::GetArticleOneQuery,
    ) -> Result<dto::ArticleDTO, ArticleServiceError> {
        let identifier = query.identifier;
        let public_only = query.public_only;
        let with_content = query.with_content;

        let retry_handler = MaxRetryAsyncHandler {
            max_retry: 3,
            should_retry: |err: &ArticleServiceError| {
                if !err.is_repository_error() {
                    return false;
                }
                let err = err.as_repository_error().unwrap();
                if err.is_article_repository_error() {
                    return false;
                }
                true
            },
        };

        let task = || async {
            let article = self
                .article_repository
                .get_one(identifier.clone(), public_only, with_content)
                .await
                .map_err(|why| ArticleServiceError::Repository(why.into()))?;

            let series = match &article.series_id {
                None => None,
                Some(id) => Some(
                    self.series_repository
                        .get_one(id.as_identifier())
                        .await
                        .map_err(|why| ArticleServiceError::Repository(why.into()))?,
                ),
            };

            let categories = match article.category_ids.is_empty() {
                true => vec![],
                false => {
                    let filter = CategoriesFilter::new(article.category_ids.clone());
                    self.category_repository
                        .get_many(Some(filter), None)
                        .await
                        .map_err(|why| ArticleServiceError::Repository(why.into()))?
                }
            };

            let tags = match article.tag_ids.is_empty() {
                true => vec![],
                false => {
                    let filter = TagsFilter::new(article.tag_ids.clone());
                    self.tag_repository
                        .get_many(Some(filter), None)
                        .await
                        .map_err(|why| ArticleServiceError::Repository(why.into()))?
                }
            };

            let mut article_dto_builder = dto::ArticleDTOBuilder::new().with_article(article);
            if let Some(series) = series {
                article_dto_builder = article_dto_builder.with_series(series);
            }
            let article_dto = article_dto_builder
                .with_categories(categories)
                .with_tags(tags)
                .try_build()
                .map_err(|why| ArticleServiceError::DTOConversion(why.into()))?;

            Ok(article_dto)
        };

        let res = retry_handler.run(task).await?;

        Ok(res)
    }

    pub async fn get_article_content(
        &self,
        query: query::GetArticleContentQuery,
    ) -> Result<String, ArticleServiceError> {
        let identifier = query.identifier;
        let public_only = query.public_only;
        let article = self
            .article_repository
            .get_one(identifier, public_only, true)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        let content = match article.content {
            None => String::from(""),
            Some(content) => content.value().clone(),
        };
        Ok(content)
    }

    pub async fn get_article_many(
        &self,
        query: query::GetArticleManyQuery,
    ) -> Result<Vec<dto::ArticleDTO>, ArticleServiceError> {
        let pagination = query.pagination;
        let public_only = query.public_only;
        let with_content = query.with_content;
        let filter = query.filter;

        let articles = self
            .article_repository
            .get_many(pagination, public_only, with_content, filter)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        let mut articles_dto = vec![];
        for article in articles {
            let series = match &article.series_id {
                None => None,
                Some(id) => Some(
                    self.series_repository
                        .get_one(id.as_identifier())
                        .await
                        .map_err(|why| ArticleServiceError::Repository(why.into()))?,
                ),
            };

            let filter = CategoriesFilter::new(article.category_ids.clone());
            let categories = self
                .category_repository
                .get_many(Some(filter), None)
                .await
                .map_err(|why| ArticleServiceError::Repository(why.into()))?;

            let filter = TagsFilter::new(article.tag_ids.clone());
            let tags = self
                .tag_repository
                .get_many(Some(filter), None)
                .await
                .map_err(|why| ArticleServiceError::Repository(why.into()))?;

            let mut article_dto_builder = dto::ArticleDTOBuilder::new().with_article(article);
            if let Some(series) = series {
                article_dto_builder = article_dto_builder.with_series(series);
            }
            let article_dto = article_dto_builder
                .with_categories(categories)
                .with_tags(tags)
                .try_build()
                .map_err(|why| ArticleServiceError::DTOConversion(why.into()))?;
            articles_dto.push(article_dto);
        }
        Ok(articles_dto)
    }

    pub async fn get_series_one(
        &self,
        query: query::GetSeriesOneQuery,
    ) -> Result<dto::SeriesDTO, ArticleServiceError> {
        let identifier = query.identifier;

        let series = self
            .series_repository
            .get_one(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        let series = series.into();

        Ok(series)
    }

    pub async fn get_series_many(
        &self,
        query: query::GetSeriesManyQuery,
    ) -> Result<Vec<dto::SeriesDTO>, ArticleServiceError> {
        let filter = query.filter;
        let pagination = query.pagination;

        let series = self
            .series_repository
            .get_many(filter, pagination)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        let series_dto = series.into_iter().map(|s| s.into()).collect();
        Ok(series_dto)
    }

    pub async fn get_category_one(
        &self,
        query: query::GetCategoryOneQuery,
    ) -> Result<dto::CategoryDTO, ArticleServiceError> {
        let identifier = query.identifier;

        let category = self
            .category_repository
            .get_one(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        let category = category.into();
        Ok(category)
    }

    pub async fn get_category_many(
        &self,
        query: query::GetCategoryManyQuery,
    ) -> Result<Vec<dto::CategoryDTO>, ArticleServiceError> {
        let filter = query.filter;
        let pagination = query.pagination;
        let categories = self
            .category_repository
            .get_many(filter, pagination)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        let categories_dto = categories.into_iter().map(|c| c.into()).collect();
        Ok(categories_dto)
    }

    pub async fn get_tag_one(
        &self,
        query: query::GetTagOneQuery,
    ) -> Result<dto::TagDTO, ArticleServiceError> {
        let identifier = query.identifier;

        let tag = self
            .tag_repository
            .get_one(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        let tag = tag.into();
        Ok(tag)
    }

    pub async fn get_tag_many(
        &self,
        query: query::GetTagManyQuery,
    ) -> Result<Vec<dto::TagDTO>, ArticleServiceError> {
        let filter = query.filter;
        let pagination = query.pagination;
        let tags = self
            .tag_repository
            .get_many(filter, pagination)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        let tags_dto = tags.into_iter().map(|t| t.into()).collect();
        Ok(tags_dto)
    }

    pub async fn publish_article(
        &self,
        command: command::PublishArticleCommand,
    ) -> Result<(), ArticleServiceError> {
        let identifier = command.identifier;

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

    pub async fn unpublish_article(
        &self,
        command: command::UnpublishArticleCommand,
    ) -> Result<(), ArticleServiceError> {
        let identifier = command.identifier;

        let mut article = self
            .article_repository
            .get_one(identifier, false, false)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        article
            .unpublish()
            .map_err(|why| ArticleServiceError::Domain(why.into()))?;

        self.article_repository
            .unpublish(article.id, article.version)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(())
    }

    pub async fn soft_delete_article(
        &self,
        command: command::SoftDeleteArticleCommand,
    ) -> Result<(), ArticleServiceError> {
        let identifier = command.identifier;

        let mut article = self
            .article_repository
            .get_one(identifier, false, false)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        article
            .soft_delete()
            .map_err(|why| ArticleServiceError::Domain(why.into()))?;

        self.article_repository
            .soft_delete(article.id, article.deleted_at.unwrap(), article.version)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(())
    }

    pub async fn revoke_soft_delete_article(
        &self,
        command: command::RevokeSoftDeleteArticleCommand,
    ) -> Result<(), ArticleServiceError> {
        let identifier = command.identifier;

        let mut article = self
            .article_repository
            .get_one(identifier, false, false)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        article
            .revoke_soft_delete()
            .map_err(|why| ArticleServiceError::Domain(why.into()))?;

        self.article_repository
            .revoke_soft_delete(article.id, article.version)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(())
    }

    pub async fn delete_article(
        &self,
        command: command::DeleteArticleCommand,
    ) -> Result<(), ArticleServiceError> {
        let identifier = command.identifier;

        self.article_repository
            .delete(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;

        Ok(())
    }

    pub async fn delete_series(
        &self,
        command: command::DeleteSeriesCommand,
    ) -> Result<(), ArticleServiceError> {
        let identifier = command.identifier;
        self.series_repository
            .delete(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(())
    }

    pub async fn delete_category(
        &self,
        command: command::DeleteCategoryCommand,
    ) -> Result<(), ArticleServiceError> {
        let identifier = command.identifier;
        self.category_repository
            .delete(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(())
    }

    pub async fn delete_tag(
        &self,
        command: command::DeleteTagCommand,
    ) -> Result<(), ArticleServiceError> {
        let identifier = command.identifier;
        self.tag_repository
            .delete(identifier)
            .await
            .map_err(|why| ArticleServiceError::Repository(why.into()))?;
        Ok(())
    }
}
