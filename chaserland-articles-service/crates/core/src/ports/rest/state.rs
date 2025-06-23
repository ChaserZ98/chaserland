use crate::app::interface::ArticleService;

#[derive(Clone)]
pub struct AppState<T>
where
    T: ArticleService,
{
    pub article_service: T,
}
