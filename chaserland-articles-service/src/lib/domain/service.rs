use super::model::article::Article;
use super::model::article::GetArticleError;
use tonic::async_trait;

#[async_trait]
pub trait ArticleService {
    async fn get_article(
        &self,
        identifier: String,
        public_only: bool,
        with_content: bool,
    ) -> Result<Article, GetArticleError>;
}
