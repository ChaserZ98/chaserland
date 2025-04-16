use crate::domain::repository::{
    article::ArticleRepository, category::CategoryRepository, series::SeriesRepository,
    tag::TagRepository,
};

pub struct ArticleAppService<R, S, C, T>
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

impl<R, S, C, T> ArticleAppService<R, S, C, T>
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
