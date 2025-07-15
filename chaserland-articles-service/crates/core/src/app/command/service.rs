use super::{command, error, interface::ArticleCommandService as ArticleCommandServiceInterface};
use crate::domain::{
    article::{
        error::ArticleDomainError,
        repository::{ArticleRepository, ArticleRepositoryError},
        vo as article,
    },
    category::{repository::CategoryRepository, vo as category},
    series::{repository::SeriesRepository, vo as series},
    tag::{repository::TagRepository, vo as tag},
};
use sqlx::{Database, Pool};

pub struct ArticleCommandService<R, S, C, T, DB>
where
    R: ArticleRepository<DB = DB>,
    S: SeriesRepository<DB = DB>,
    C: CategoryRepository<DB = DB>,
    T: TagRepository<DB = DB>,
    DB: Database,
{
    article_repository: R,
    series_repository: S,
    category_repository: C,
    tag_repository: T,
    pool: Pool<DB>,
}

impl<R, S, C, T, DB> ArticleCommandService<R, S, C, T, DB>
where
    R: ArticleRepository<DB = DB>,
    S: SeriesRepository<DB = DB>,
    C: CategoryRepository<DB = DB>,
    T: TagRepository<DB = DB>,
    DB: Database,
{
    pub fn new(
        article_repository: R,
        series_repository: S,
        category_repository: C,
        tag_repository: T,
        pool: Pool<DB>,
    ) -> Self {
        Self {
            article_repository,
            series_repository,
            category_repository,
            tag_repository,
            pool,
        }
    }
}

impl<R, S, C, T, DB> Clone for ArticleCommandService<R, S, C, T, DB>
where
    R: ArticleRepository<DB = DB>,
    S: SeriesRepository<DB = DB>,
    C: CategoryRepository<DB = DB>,
    T: TagRepository<DB = DB>,
    DB: Database,
{
    fn clone(&self) -> Self {
        Self {
            article_repository: self.article_repository.clone(),
            series_repository: self.series_repository.clone(),
            category_repository: self.category_repository.clone(),
            tag_repository: self.tag_repository.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl<R, S, C, T, DB> ArticleCommandServiceInterface for ArticleCommandService<R, S, C, T, DB>
where
    R: ArticleRepository<DB = DB>,
    S: SeriesRepository<DB = DB>,
    C: CategoryRepository<DB = DB>,
    T: TagRepository<DB = DB>,
    DB: Database,
{
    async fn create_article(
        &self,
        command: command::CreateArticleCommand,
    ) -> Result<(), error::CreateArticleError> {
        let mut tx = self.pool.begin().await?;

        let new_article = article::NewArticle::new(
            command.title,
            command.description,
            command.content,
            command.series_id,
            command.category_ids,
            command.tag_ids,
        );
        let mut categories = vec![];
        for category_id in &new_article.category_ids {
            let category = self
                .category_repository
                .get_one(category_id.as_identifier(), &mut tx)
                .await?;
            categories.push(category);
        }

        let mut tags = vec![];
        for tag_id in &new_article.tag_ids {
            let tag = self
                .tag_repository
                .get_one(tag_id.as_identifier(), &mut tx)
                .await?;
            tags.push(tag);
        }

        self.article_repository.create(new_article, &mut tx).await?;

        tx.commit().await?;

        Ok(())
    }

    async fn create_series(
        &self,
        command: command::CreateSeriesCommand,
    ) -> Result<(), error::CreateSeriesError> {
        let mut tx = self.pool.begin().await?;

        let new_series = series::NewSeries::new(command.name);
        self.series_repository.create(new_series, &mut tx).await?;

        tx.commit().await?;

        Ok(())
    }

    async fn create_category(
        &self,
        command: command::CreateCategoryCommand,
    ) -> Result<(), error::CreateCategoryError> {
        let mut tx = self.pool.begin().await?;

        let new_category = category::NewCategory::new(command.name);
        self.category_repository
            .create(new_category, &mut tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn create_tag(
        &self,
        command: command::CreateTagCommand,
    ) -> Result<(), error::CreateTagError> {
        let mut tx = self.pool.begin().await?;

        let new_tag = tag::NewTag::new(command.name);
        self.tag_repository.create(new_tag, &mut tx).await?;

        tx.commit().await?;

        Ok(())
    }

    async fn publish_article(
        &self,
        command: command::PublishArticleCommand,
    ) -> Result<(), error::PublishArticleError> {
        let mut tx = self.pool.begin().await?;

        let identifier = command.identifier;

        let (mut article, version) = self
            .article_repository
            .get_one(identifier.clone(), &mut tx)
            .await?;

        article.publish().map_err(|e| match e {
            ArticleDomainError::AlreadyPublished(_) => {
                error::PublishArticleError::AlreadyPublished(identifier)
            }
            _ => e.into(),
        })?;

        self.article_repository
            .publish(
                article.id,
                article.published_at.clone().unwrap(),
                version,
                &mut tx,
            )
            .await
            .map_err(|e| match e {
                ArticleRepositoryError::ArticleNotFound(_)
                | ArticleRepositoryError::ConcurrentConflict { .. } => {
                    error::PublishArticleError::DataVersionConflict(e.to_string())
                }
                _ => e.into(),
            })?;

        tx.commit().await?;

        Ok(())
    }

    async fn unpublish_article(
        &self,
        command: command::UnpublishArticleCommand,
    ) -> Result<(), error::UnpublishArticleError> {
        let mut tx = self.pool.begin().await?;

        let identifier = command.identifier;

        let (mut article, version) = self
            .article_repository
            .get_one(identifier.clone(), &mut tx)
            .await?;

        article.unpublish().map_err(|e| match e {
            ArticleDomainError::NotPublished(_) => {
                error::UnpublishArticleError::NotPublished(identifier)
            }
            _ => error::UnpublishArticleError::Domain(e.into()),
        })?;

        self.article_repository
            .unpublish(article.id, version, &mut tx)
            .await
            .map_err(|e| match e {
                ArticleRepositoryError::ArticleNotFound(_)
                | ArticleRepositoryError::ConcurrentConflict { .. } => {
                    error::UnpublishArticleError::DataVersionConflict(e.to_string())
                }
                _ => error::UnpublishArticleError::Repository(e.into()),
            })?;

        tx.commit().await?;

        Ok(())
    }

    async fn soft_delete_article(
        &self,
        command: command::SoftDeleteArticleCommand,
    ) -> Result<(), error::SoftDeleteArticleError> {
        let mut tx = self.pool.begin().await?;

        let identifier = command.identifier;

        let (mut article, version) = self
            .article_repository
            .get_one(identifier.clone(), &mut tx)
            .await?;

        article.soft_delete().map_err(|e| match e {
            ArticleDomainError::AlreadySoftDeleted(_) => {
                error::SoftDeleteArticleError::AlreadySoftDeleted(identifier)
            }
            _ => e.into(),
        })?;

        self.article_repository
            .soft_delete(article.id, article.deleted_at.unwrap(), version, &mut tx)
            .await
            .map_err(|e| match e {
                ArticleRepositoryError::ArticleNotFound(_)
                | ArticleRepositoryError::ConcurrentConflict { .. } => {
                    error::SoftDeleteArticleError::DataVersionConflict(e.to_string())
                }
                _ => e.into(),
            })?;

        tx.commit().await?;

        Ok(())
    }

    async fn revoke_soft_delete_article(
        &self,
        command: command::RevokeSoftDeleteArticleCommand,
    ) -> Result<(), error::RevokeSoftDeleteError> {
        let mut tx = self.pool.begin().await?;

        let identifier = command.identifier;

        let (mut article, version) = self
            .article_repository
            .get_one(identifier.clone(), &mut tx)
            .await?;

        article.revoke_soft_delete().map_err(|e| match e {
            ArticleDomainError::NotSoftDeleted(_) => {
                error::RevokeSoftDeleteError::NotSoftDeleted(identifier)
            }
            _ => e.into(),
        })?;

        self.article_repository
            .revoke_soft_delete(article.id, version, &mut tx)
            .await
            .map_err(|e| match e {
                ArticleRepositoryError::ArticleNotFound(_)
                | ArticleRepositoryError::ConcurrentConflict { .. } => {
                    error::RevokeSoftDeleteError::DataVersionConflict(e.to_string())
                }
                _ => e.into(),
            })?;

        tx.commit().await?;

        Ok(())
    }

    async fn delete_article(
        &self,
        command: command::DeleteArticleCommand,
    ) -> Result<(), error::DeleteArticleError> {
        let mut tx = self.pool.begin().await?;

        let identifier = command.identifier;

        self.article_repository.delete(identifier, &mut tx).await?;

        tx.commit().await?;

        Ok(())
    }

    async fn delete_series(
        &self,
        command: command::DeleteSeriesCommand,
    ) -> Result<(), error::DeleteSeriesError> {
        let mut tx = self.pool.begin().await?;

        let identifier = command.identifier;
        self.series_repository.delete(identifier, &mut tx).await?;

        tx.commit().await?;

        Ok(())
    }

    async fn delete_category(
        &self,
        command: command::DeleteCategoryCommand,
    ) -> Result<(), error::DeleteCategoryError> {
        let mut tx = self.pool.begin().await?;

        let identifier = command.identifier;
        self.category_repository.delete(identifier, &mut tx).await?;

        tx.commit().await?;

        Ok(())
    }

    async fn delete_tag(
        &self,
        command: command::DeleteTagCommand,
    ) -> Result<(), error::DeleteTagError> {
        let mut tx = self.pool.begin().await?;

        let identifier = command.identifier;
        self.tag_repository.delete(identifier, &mut tx).await?;

        tx.commit().await?;

        Ok(())
    }
}
