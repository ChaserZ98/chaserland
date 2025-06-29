use crate::app::interface::ArticleService;

#[derive(Clone)]
pub struct AppState<T>
where
    T: ArticleService,
{
    pub article_service: T,
}

impl<T> AppState<T>
where
    T: ArticleService,
{
    pub fn new(article_service: T) -> Self {
        Self { article_service }
    }
}
