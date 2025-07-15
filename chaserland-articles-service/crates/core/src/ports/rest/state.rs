use crate::app::{
    command::interface::ArticleCommandService, query::interface::ArticleQueryService,
};

#[derive(Clone)]
pub struct AppState<C, Q>
where
    C: ArticleCommandService,
    Q: ArticleQueryService,
{
    pub article_command_service: C,
    pub article_query_service: Q,
}

impl<C, Q> AppState<C, Q>
where
    C: ArticleCommandService,
    Q: ArticleQueryService,
{
    pub fn new(article_command_service: C, article_query_service: Q) -> Self {
        Self {
            article_command_service,
            article_query_service,
        }
    }
}
