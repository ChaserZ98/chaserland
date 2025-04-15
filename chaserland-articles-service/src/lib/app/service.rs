use crate::domain::repository::ArticleRepository;

pub struct ArticleAppService<R>
where
    R: ArticleRepository,
{
    repository: R,
}

impl<R> ArticleAppService<R>
where
    R: ArticleRepository,
{
    pub fn new(repository: R) -> Self {
        ArticleAppService { repository }
    }
}
