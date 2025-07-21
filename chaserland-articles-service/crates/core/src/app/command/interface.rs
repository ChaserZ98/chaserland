use super::{command, error};

#[trait_variant::make(ArticleCommandService: Send)]
pub trait LocalArticleCommandService: Clone + Sync + 'static {
    async fn create_article(
        &self,
        command: command::CreateArticleCommand,
    ) -> Result<(), error::CreateArticleError>;

    async fn create_series(
        &self,
        command: command::CreateSeriesCommand,
    ) -> Result<(), error::CreateSeriesError>;

    async fn create_category(
        &self,
        command: command::CreateCategoryCommand,
    ) -> Result<(), error::CreateCategoryError>;

    async fn create_tag(
        &self,
        command: command::CreateTagCommand,
    ) -> Result<(), error::CreateTagError>;

    async fn update_article(
        &self,
        command: command::UpdateArticleCommand,
    ) -> Result<(), error::UpdateArticleError>;

    async fn publish_article(
        &self,
        command: command::PublishArticleCommand,
    ) -> Result<(), error::PublishArticleError>;

    async fn unpublish_article(
        &self,
        command: command::UnpublishArticleCommand,
    ) -> Result<(), error::UnpublishArticleError>;

    async fn soft_delete_article(
        &self,
        command: command::SoftDeleteArticleCommand,
    ) -> Result<(), error::SoftDeleteArticleError>;

    async fn revoke_soft_delete_article(
        &self,
        command: command::RevokeSoftDeleteArticleCommand,
    ) -> Result<(), error::RevokeSoftDeleteError>;

    async fn delete_article(
        &self,
        command: command::DeleteArticleCommand,
    ) -> Result<(), error::DeleteArticleError>;

    async fn delete_series(
        &self,
        command: command::DeleteSeriesCommand,
    ) -> Result<(), error::DeleteSeriesError>;

    async fn delete_category(
        &self,
        command: command::DeleteCategoryCommand,
    ) -> Result<(), error::DeleteCategoryError>;

    async fn delete_tag(
        &self,
        command: command::DeleteTagCommand,
    ) -> Result<(), error::DeleteTagError>;
}
