use super::{
    command, error,
    interface::{
        ArticleCommandService as ArticleCommandServiceInterface,
        ArticleQueryService as ArticleQueryServiceInterface,
    },
    query,
};
use crate::{
    app::dto,
    domain::{
        article::{
            error::ArticleDomainError,
            repository::{ArticleRepository, ArticleRepositoryError},
            vo as article,
        },
        category::{
            repository::{CategoriesFilter, CategoryRepository},
            vo as category,
        },
        series::{repository::SeriesRepository, vo as series},
        tag::{
            repository::{TagRepository, TagsFilter},
            vo as tag,
        },
    },
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

#[derive(Clone)]
pub struct ArticleCommandService<R, S, C, T>
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

impl<R, S, C, T> ArticleCommandService<R, S, C, T>
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
}

impl<R, S, C, T> ArticleCommandServiceInterface for ArticleCommandService<R, S, C, T>
where
    R: ArticleRepository,
    S: SeriesRepository,
    C: CategoryRepository,
    T: TagRepository,
{
    async fn create_article(
        &self,
        command: command::CreateArticleCommand,
    ) -> Result<dto::ArticleDTO, error::CreateArticleError> {
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
            Some(id) => Some(self.series_repository.get_one(id.as_identifier()).await?),
        };
        let mut categories = vec![];
        for category_id in &new_article.category_ids {
            let category = self
                .category_repository
                .get_one(category_id.as_identifier())
                .await?;
            categories.push(category);
        }

        let mut tags = vec![];
        for tag_id in &new_article.tag_ids {
            let tag = self.tag_repository.get_one(tag_id.as_identifier()).await?;
            tags.push(tag);
        }

        let article = self.article_repository.create(new_article).await?;

        let mut article_dto_builder = dto::ArticleDTOBuilder::new().with_article(article);
        if let Some(series) = series {
            article_dto_builder = article_dto_builder.with_series(series);
        }
        article_dto_builder = article_dto_builder
            .with_categories(categories)
            .with_tags(tags);

        let article_dto = article_dto_builder.try_build()?;

        Ok(article_dto)
    }

    async fn create_series(
        &self,
        command: command::CreateSeriesCommand,
    ) -> Result<dto::SeriesDTO, error::CreateSeriesError> {
        let new_series = series::NewSeries::new(command.name);
        let series = self.series_repository.create(new_series).await?;
        let series_dto = series.into();
        Ok(series_dto)
    }

    async fn create_category(
        &self,
        command: command::CreateCategoryCommand,
    ) -> Result<dto::CategoryDTO, error::CreateCategoryError> {
        let new_category = category::NewCategory::new(command.name);
        let category = self.category_repository.create(new_category).await?;
        let category_dto = category.into();
        Ok(category_dto)
    }

    async fn create_tag(
        &self,
        command: command::CreateTagCommand,
    ) -> Result<dto::TagDTO, error::CreateTagError> {
        let new_tag = tag::NewTag::new(command.name);
        let tag = self.tag_repository.create(new_tag).await?;
        let tag_dto = tag.into();
        Ok(tag_dto)
    }

    async fn get_article_one(
        &self,
        query: query::GetArticleOneQuery,
    ) -> Result<dto::ArticleDTO, error::GetArticleOneError> {
        let identifier = query.identifier;
        let public_only = query.public_only;
        let with_content = query.with_content;

        // let retry_handler = MaxRetryAsyncHandler {
        //     max_retry: 3,
        //     should_retry: |err: &error::GetArticleOneError| {
        //         if !err.is_repository_error() {
        //             return false;
        //         }
        //         let err = err.as_repository_error().unwrap();
        //         if err.is_article_repository_error() {
        //             return false;
        //         }
        //         true
        //     },
        // };

        let (article, _) = self
            .article_repository
            .get_one(identifier.clone(), public_only, with_content)
            .await?;

        let series = match &article.series_id {
            None => None,
            Some(id) => {
                let series = self.series_repository.get_one(id.as_identifier()).await?;
                Some(series)
            }
        };

        let categories = match article.category_ids.is_empty() {
            true => vec![],
            false => {
                let filter = CategoriesFilter::new(article.category_ids.clone());
                self.category_repository
                    .get_many(Some(filter), None)
                    .await?
            }
        };

        let tags = match article.tag_ids.is_empty() {
            true => vec![],
            false => {
                let filter = TagsFilter::new(article.tag_ids.clone());
                self.tag_repository.get_many(Some(filter), None).await?
            }
        };

        let mut article_dto_builder = dto::ArticleDTOBuilder::new().with_article(article);
        if let Some(series) = series {
            article_dto_builder = article_dto_builder.with_series(series);
        }
        let article_dto = article_dto_builder
            .with_categories(categories)
            .with_tags(tags)
            .try_build()?;

        // let res = retry_handler.run(task).await?;

        Ok(article_dto)
    }

    async fn get_article_content(
        &self,
        query: query::GetArticleContentQuery,
    ) -> Result<String, error::GetArticleContentError> {
        let identifier = query.identifier;
        let public_only = query.public_only;
        let (article, _) = self
            .article_repository
            .get_one(identifier, public_only, true)
            .await?;

        let content = match article.content {
            None => String::from(""),
            Some(content) => content.value().clone(),
        };
        Ok(content)
    }

    async fn get_article_many(
        &self,
        query: query::GetArticleManyQuery,
    ) -> Result<Vec<dto::ArticleDTO>, error::GetArticleManyError> {
        let pagination = query.pagination;
        let public_only = query.public_only;
        let with_content = query.with_content;
        let filter = query.filter;

        let articles = self
            .article_repository
            .get_many(pagination, public_only, with_content, filter)
            .await?;

        let mut articles_dto = vec![];
        for article in articles {
            let series = match &article.series_id {
                None => None,
                Some(id) => {
                    let series = self.series_repository.get_one(id.as_identifier()).await?;
                    Some(series)
                }
            };

            let filter = CategoriesFilter::new(article.category_ids.clone());
            let categories = self
                .category_repository
                .get_many(Some(filter), None)
                .await?;

            let filter = TagsFilter::new(article.tag_ids.clone());
            let tags = self.tag_repository.get_many(Some(filter), None).await?;

            let mut article_dto_builder = dto::ArticleDTO::builder().with_article(article);
            if let Some(series) = series {
                article_dto_builder = article_dto_builder.with_series(series);
            }
            let article_dto = article_dto_builder
                .with_categories(categories)
                .with_tags(tags)
                .try_build()?;
            articles_dto.push(article_dto);
        }
        Ok(articles_dto)
    }

    async fn get_series_one(
        &self,
        query: query::GetSeriesOneQuery,
    ) -> Result<dto::SeriesDTO, error::GetSeriesOneError> {
        let identifier = query.identifier;

        let series = self.series_repository.get_one(identifier).await?;

        let series = series.into();

        Ok(series)
    }

    async fn get_series_many(
        &self,
        query: query::GetSeriesManyQuery,
    ) -> Result<Vec<dto::SeriesDTO>, error::GetSeriesManyError> {
        let filter = query.filter;
        let pagination = query.pagination;

        let series = self.series_repository.get_many(filter, pagination).await?;

        let series_dto = series.into_iter().map(|s| s.into()).collect();
        Ok(series_dto)
    }

    async fn get_category_one(
        &self,
        query: query::GetCategoryOneQuery,
    ) -> Result<dto::CategoryDTO, error::GetCategoryOneError> {
        let identifier = query.identifier;

        let category = self.category_repository.get_one(identifier).await?;

        let category = category.into();
        Ok(category)
    }

    async fn get_category_many(
        &self,
        query: query::GetCategoryManyQuery,
    ) -> Result<Vec<dto::CategoryDTO>, error::GetCategoryManyError> {
        let filter = query.filter;
        let pagination = query.pagination;
        let categories = self
            .category_repository
            .get_many(filter, pagination)
            .await?;
        let categories_dto = categories.into_iter().map(|c| c.into()).collect();
        Ok(categories_dto)
    }

    async fn get_tag_one(
        &self,
        query: query::GetTagOneQuery,
    ) -> Result<dto::TagDTO, error::GetTagOneError> {
        let identifier = query.identifier;

        let tag = self.tag_repository.get_one(identifier).await?;

        let tag = tag.into();
        Ok(tag)
    }

    async fn get_tag_many(
        &self,
        query: query::GetTagManyQuery,
    ) -> Result<Vec<dto::TagDTO>, error::GetTagManyError> {
        let filter = query.filter;
        let pagination = query.pagination;
        let tags = self.tag_repository.get_many(filter, pagination).await?;
        let tags_dto = tags.into_iter().map(|t| t.into()).collect();
        Ok(tags_dto)
    }

    async fn publish_article(
        &self,
        command: command::PublishArticleCommand,
    ) -> Result<(), error::PublishArticleError> {
        let identifier = command.identifier;

        let (mut article, version) = self
            .article_repository
            .get_one(identifier.clone(), false, false)
            .await?;

        article.publish().map_err(|e| match e {
            ArticleDomainError::AlreadyPublished(_) => {
                error::PublishArticleError::AlreadyPublished(identifier)
            }
            _ => error::PublishArticleError::Domain(e.into()),
        })?;

        self.article_repository
            .publish(article.id, article.published_at.clone().unwrap(), version)
            .await
            .map_err(|e| match e {
                ArticleRepositoryError::ArticleNotFound(_)
                | ArticleRepositoryError::VersionMismatch { .. } => {
                    error::PublishArticleError::DataVersionConflict(e.to_string())
                }
                _ => error::PublishArticleError::Repository(e.into()),
            })?;
        Ok(())
    }

    async fn unpublish_article(
        &self,
        command: command::UnpublishArticleCommand,
    ) -> Result<(), error::UnpublishArticleError> {
        let identifier = command.identifier;

        let (mut article, version) = self
            .article_repository
            .get_one(identifier.clone(), false, false)
            .await?;

        article.unpublish().map_err(|e| match e {
            ArticleDomainError::NotPublished(_) => {
                error::UnpublishArticleError::NotPublished(identifier)
            }
            _ => error::UnpublishArticleError::Domain(e.into()),
        })?;

        self.article_repository
            .unpublish(article.id, version)
            .await
            .map_err(|e| match e {
                ArticleRepositoryError::ArticleNotFound(_)
                | ArticleRepositoryError::VersionMismatch { .. } => {
                    error::UnpublishArticleError::DataVersionConflict(e.to_string())
                }
                _ => error::UnpublishArticleError::Repository(e.into()),
            })?;
        Ok(())
    }

    async fn soft_delete_article(
        &self,
        command: command::SoftDeleteArticleCommand,
    ) -> Result<(), error::SoftDeleteArticleError> {
        let identifier = command.identifier;

        let (mut article, version) = self
            .article_repository
            .get_one(identifier.clone(), false, false)
            .await?;

        article.soft_delete().map_err(|e| match e {
            ArticleDomainError::AlreadySoftDeleted(_) => {
                error::SoftDeleteArticleError::AlreadySoftDeleted(identifier)
            }
            _ => error::SoftDeleteArticleError::Domain(e.into()),
        })?;

        self.article_repository
            .soft_delete(article.id, article.deleted_at.unwrap(), version)
            .await
            .map_err(|e| match e {
                ArticleRepositoryError::ArticleNotFound(_)
                | ArticleRepositoryError::VersionMismatch { .. } => {
                    error::SoftDeleteArticleError::DataVersionConflict(e.to_string())
                }
                _ => error::SoftDeleteArticleError::Repository(e.into()),
            })?;
        Ok(())
    }

    async fn revoke_soft_delete_article(
        &self,
        command: command::RevokeSoftDeleteArticleCommand,
    ) -> Result<(), error::RevokeSoftDeleteError> {
        let identifier = command.identifier;

        let (mut article, version) = self
            .article_repository
            .get_one(identifier.clone(), false, false)
            .await?;

        article.revoke_soft_delete().map_err(|e| match e {
            ArticleDomainError::NotSoftDeleted(_) => {
                error::RevokeSoftDeleteError::NotSoftDeleted(identifier)
            }
            _ => error::RevokeSoftDeleteError::Domain(e.into()),
        })?;

        self.article_repository
            .revoke_soft_delete(article.id, version)
            .await
            .map_err(|e| match e {
                ArticleRepositoryError::ArticleNotFound(_)
                | ArticleRepositoryError::VersionMismatch { .. } => {
                    error::RevokeSoftDeleteError::DataVersionConflict(e.to_string())
                }
                _ => error::RevokeSoftDeleteError::Repository(e.into()),
            })?;
        Ok(())
    }

    async fn delete_article(
        &self,
        command: command::DeleteArticleCommand,
    ) -> Result<(), error::DeleteArticleError> {
        let identifier = command.identifier;

        self.article_repository.delete(identifier).await?;

        Ok(())
    }

    async fn delete_series(
        &self,
        command: command::DeleteSeriesCommand,
    ) -> Result<(), error::DeleteSeriesError> {
        let identifier = command.identifier;
        self.series_repository.delete(identifier).await?;
        Ok(())
    }

    async fn delete_category(
        &self,
        command: command::DeleteCategoryCommand,
    ) -> Result<(), error::DeleteCategoryError> {
        let identifier = command.identifier;
        self.category_repository.delete(identifier).await?;
        Ok(())
    }

    async fn delete_tag(
        &self,
        command: command::DeleteTagCommand,
    ) -> Result<(), error::DeleteTagError> {
        let identifier = command.identifier;
        self.tag_repository.delete(identifier).await?;
        Ok(())
    }
}

pub struct ArticleQueryService<A>
where
    A: Article,
{
    article_query_handler: A,
}

// impl<T> ArticleQueryService<T>
// where
//     T: sqlx::Database,
// {
//     pub fn new(db_pool: sqlx::Pool<T>) -> Self {
//         Self { db_pool }
//     }
// }

impl<T> ArticleQueryServiceInterface for ArticleQueryService<T>
where
    T: sqlx::Database,
{
    async fn get_article_one(
        &self,
        query: query::GetArticleOneQuery,
    ) -> Result<dto::ArticleDTO, error::GetArticleOneError> {
        todo!()
    }
    async fn get_article_content(
        &self,
        query: query::GetArticleContentQuery,
    ) -> Result<String, error::GetArticleContentError> {
        todo!()
    }
    async fn get_article_many_(
        &self,
        query: query::GetArticleManyQuery,
    ) -> Result<Vec<dto::ArticleDTO>, error::GetArticleManyError> {
        todo!()
    }
}
