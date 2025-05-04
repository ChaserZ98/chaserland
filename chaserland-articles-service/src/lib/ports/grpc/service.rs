use crate::app::service::ArticleService;
use crate::domain::repository::article::ArticleRepository;
use crate::domain::repository::category::CategoryRepository;
use crate::domain::repository::series::SeriesRepository;
use crate::domain::repository::tag::TagRepository;
use chaserland_protos::article::v1::{
    CreateArticleRequest, CreateArticleResponse, CreateCategoryRequest, CreateCategoryResponse,
    CreateSeriesRequest, CreateSeriesResponse, CreateTagRequest, CreateTagResponse,
    DeleteArticleRequest, DeleteArticleResponse, DeleteCategoryRequest, DeleteCategoryResponse,
    DeleteSeriesRequest, DeleteSeriesResponse, DeleteTagRequest, DeleteTagResponse,
    GetArticleContentRequest, GetArticleContentResponse, GetArticleManyRequest,
    GetArticleManyResponse, GetArticleOneRequest, GetArticleOneResponse, GetCategoryManyRequest,
    GetCategoryManyResponse, GetCategoryOneRequest, GetCategoryOneResponse, GetSeriesManyRequest,
    GetSeriesManyResponse, GetSeriesOneRequest, GetSeriesOneResponse, GetTagManyRequest,
    GetTagManyResponse, GetTagOneRequest, GetTagOneResponse, PublishArticleRequest,
    PublishArticleResponse, RevokeSoftDeleteArticleRequest, RevokeSoftDeleteArticleResponse,
    SoftDeleteArticleRequest, SoftDeleteArticleResponse, UnpublishArticleRequest,
    UnpublishArticleResponse,
    article_service_server::{ArticleService as TonicArticleService, ArticleServiceServer},
};
use tonic::{Request, Response, Status};

pub struct GrpcArticleService<
    R: ArticleRepository,
    S: SeriesRepository,
    C: CategoryRepository,
    T: TagRepository,
>(ArticleService<R, S, C, T>);

impl<R, S, C, T> Into<GrpcArticleService<R, S, C, T>> for ArticleService<R, S, C, T>
where
    R: ArticleRepository,
    S: SeriesRepository,
    C: CategoryRepository,
    T: TagRepository,
{
    fn into(self) -> GrpcArticleService<R, S, C, T> {
        GrpcArticleService::new(self)
    }
}

impl<R, S, C, T> GrpcArticleService<R, S, C, T>
where
    R: ArticleRepository,
    S: SeriesRepository,
    C: CategoryRepository,
    T: TagRepository,
{
    pub fn new(article_service: ArticleService<R, S, C, T>) -> Self {
        Self(article_service)
    }

    pub fn inner(&self) -> &ArticleService<R, S, C, T> {
        &self.0
    }

    pub fn into_tonic_service(self) -> ArticleServiceServer<Self> {
        ArticleServiceServer::new(self)
    }
}

#[tonic::async_trait]
impl<R, S, C, T> TonicArticleService for GrpcArticleService<R, S, C, T>
where
    R: ArticleRepository,
    S: SeriesRepository,
    C: CategoryRepository,
    T: TagRepository,
{
    async fn create_article(
        &self,
        request: Request<CreateArticleRequest>,
    ) -> Result<Response<CreateArticleResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        let article = self.inner().create_article(command).await.map_err(|e| {
            if !e.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let e = e.as_repository_error().unwrap();

            if !e.is_article_repository_error() {
                return Status::internal("Internal Error");
            }

            let e = e.as_article_repository_error().unwrap();

            if e.is_duplicate_article_slug() {
                return Status::already_exists(e.to_string());
            }

            Status::internal("Internal Error")
        })?;

        let reply = CreateArticleResponse::from(article);
        Ok(Response::new(reply))
    }

    async fn create_series(
        &self,
        request: Request<CreateSeriesRequest>,
    ) -> Result<Response<CreateSeriesResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        let series = self.inner().create_series(command).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_series_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_series_repository_error().unwrap();

            if why.is_duplicate_series_slug() {
                return Status::already_exists(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        let reply = CreateSeriesResponse::from(series);

        Ok(Response::new(reply))
    }

    async fn create_category(
        &self,
        request: Request<CreateCategoryRequest>,
    ) -> Result<Response<CreateCategoryResponse>, Status> {
        let message = request.into_inner();
        let command = message.try_into()?;

        let category = self.inner().create_category(command).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_category_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_category_repository_error().unwrap();

            if why.is_duplicate_category_slug() {
                return Status::already_exists(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        let reply = CreateCategoryResponse::from(category);
        Ok(Response::new(reply))
    }

    async fn create_tag(
        &self,
        request: Request<CreateTagRequest>,
    ) -> Result<Response<CreateTagResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        let tag = self.inner().create_tag(command).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_tag_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_tag_repository_error().unwrap();

            if why.is_duplicate_tag_slug() {
                return Status::already_exists(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        let reply = CreateTagResponse::from(tag);
        Ok(Response::new(reply))
    }

    async fn get_article_one(
        &self,
        request: Request<GetArticleOneRequest>,
    ) -> Result<Response<GetArticleOneResponse>, Status> {
        let message = request.into_inner();
        let query = message.try_into()?;

        let article = self.inner().get_article_one(query).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if why.is_article_repository_error() {
                let why = why.as_article_repository_error().unwrap();

                if why.is_article_not_found() {
                    return Status::not_found(why.to_string());
                }

                return Status::internal("Internal Error");
            }

            Status::internal("Internal Error")
        })?;

        let reply = GetArticleOneResponse {
            article: Some(article.into()),
        };
        Ok(Response::new(reply))
    }

    async fn get_article_content(
        &self,
        request: Request<GetArticleContentRequest>,
    ) -> Result<Response<GetArticleContentResponse>, Status> {
        let message = request.into_inner();

        let query = message.try_into()?;

        let content = self
            .inner()
            .get_article_content(query)
            .await
            .map_err(|why| {
                if !why.is_repository_error() {
                    return Status::internal("Internal Error");
                }

                let why = why.as_repository_error().unwrap();

                if why.is_article_repository_error() {
                    let why = why.as_article_repository_error().unwrap();

                    if why.is_article_not_found() {
                        return Status::not_found(why.to_string());
                    }

                    return Status::internal("Internal Error");
                }

                Status::internal("Internal Error")
            })?;

        let reply = GetArticleContentResponse { content };
        Ok(Response::new(reply))
    }

    async fn get_article_many(
        &self,
        request: Request<GetArticleManyRequest>,
    ) -> Result<Response<GetArticleManyResponse>, Status> {
        let message = request.into_inner();
        let query = message.try_into()?;

        let articles = self
            .inner()
            .get_article_many(query)
            .await
            .map_err(|_| Status::internal("Internal Error"))?;

        let reply = GetArticleManyResponse {
            articles: articles.into_iter().map(|x| x.into()).collect(),
        };
        Ok(Response::new(reply))
    }

    async fn get_series_one(
        &self,
        request: Request<GetSeriesOneRequest>,
    ) -> Result<Response<GetSeriesOneResponse>, Status> {
        let message = request.into_inner();

        let query = message.try_into()?;

        let series = self.inner().get_series_one(query).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_series_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_series_repository_error().unwrap();

            if why.is_series_not_found() {
                return Status::not_found(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        let reply = GetSeriesOneResponse {
            series: Some(series.into()),
        };
        Ok(Response::new(reply))
    }

    async fn get_series_many(
        &self,
        request: Request<GetSeriesManyRequest>,
    ) -> Result<Response<GetSeriesManyResponse>, Status> {
        let message = request.into_inner();

        let query = message.try_into()?;

        let series = self.inner().get_series_many(query).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_series_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_series_repository_error().unwrap();

            if why.is_series_not_found() {
                return Status::not_found(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        let reply = GetSeriesManyResponse {
            series: series.into_iter().map(|x| x.into()).collect(),
        };

        Ok(Response::new(reply))
    }

    async fn get_category_one(
        &self,
        request: Request<GetCategoryOneRequest>,
    ) -> Result<Response<GetCategoryOneResponse>, Status> {
        let message = request.into_inner();

        let query = message.try_into()?;

        let category = self.inner().get_category_one(query).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_category_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_category_repository_error().unwrap();

            if why.is_category_not_found() {
                return Status::not_found(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        let reply = GetCategoryOneResponse {
            category: Some(category.into()),
        };
        Ok(Response::new(reply))
    }

    async fn get_category_many(
        &self,
        request: Request<GetCategoryManyRequest>,
    ) -> Result<Response<GetCategoryManyResponse>, Status> {
        let message = request.into_inner();

        let query = message.try_into()?;

        let categories = self.inner().get_category_many(query).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_category_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_category_repository_error().unwrap();

            if why.is_category_not_found() {
                return Status::not_found(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        let reply = GetCategoryManyResponse {
            categories: categories.into_iter().map(|x| x.into()).collect(),
        };

        Ok(Response::new(reply))
    }

    async fn get_tag_one(
        &self,
        request: Request<GetTagOneRequest>,
    ) -> Result<Response<GetTagOneResponse>, Status> {
        let message = request.into_inner();

        let query = message.try_into()?;

        let tag = self.inner().get_tag_one(query).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_tag_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_tag_repository_error().unwrap();

            if why.is_tag_not_found() {
                return Status::not_found(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        let reply = GetTagOneResponse {
            tag: Some(tag.into()),
        };
        Ok(Response::new(reply))
    }

    async fn get_tag_many(
        &self,
        request: Request<GetTagManyRequest>,
    ) -> Result<Response<GetTagManyResponse>, Status> {
        let message = request.into_inner();

        let query = message.try_into()?;

        let tags = self.inner().get_tag_many(query).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_tag_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_tag_repository_error().unwrap();

            if why.is_tag_not_found() {
                return Status::not_found(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        let reply = GetTagManyResponse {
            tags: tags.into_iter().map(|x| x.into()).collect(),
        };

        Ok(Response::new(reply))
    }

    async fn publish_article(
        &self,
        request: Request<PublishArticleRequest>,
    ) -> Result<Response<PublishArticleResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.inner().publish_article(command).await.map_err(|why| {
            if why.is_domain_error() {
                let why = why.as_domain_error().unwrap();

                if !why.is_article_domain_error() {
                    return Status::internal("Internal Error");
                }

                let why = why.as_article_domain_error().unwrap();

                if why.is_already_published() {
                    return Status::failed_precondition(why.to_string());
                }
                return Status::internal("Internal Error");
            }

            if why.is_repository_error() {
                let why = why.as_repository_error().unwrap();

                if !why.is_article_repository_error() {
                    return Status::internal("Internal Error");
                }

                let why = why.as_article_repository_error().unwrap();

                if why.is_article_not_found() {
                    return Status::not_found(why.to_string());
                }

                return Status::internal("Internal Error");
            }

            Status::internal("Internal Error")
        })?;

        Ok(Response::new(PublishArticleResponse {}))
    }

    async fn unpublish_article(
        &self,
        request: Request<UnpublishArticleRequest>,
    ) -> Result<Response<UnpublishArticleResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.inner()
            .unpublish_article(command)
            .await
            .map_err(|why| {
                if why.is_domain_error() {
                    let why = why.as_domain_error().unwrap();

                    if !why.is_article_domain_error() {
                        return Status::internal("Internal Error");
                    }

                    let why = why.as_article_domain_error().unwrap();

                    if why.is_not_published() {
                        return Status::failed_precondition(why.to_string());
                    }
                    return Status::internal("Internal Error");
                }

                if why.is_repository_error() {
                    let why = why.as_repository_error().unwrap();

                    if !why.is_article_repository_error() {
                        return Status::internal("Internal Error");
                    }

                    let why = why.as_article_repository_error().unwrap();

                    if why.is_article_not_found() {
                        return Status::not_found(why.to_string());
                    }

                    return Status::internal("Internal Error");
                }

                Status::internal("Internal Error")
            })?;

        Ok(Response::new(UnpublishArticleResponse {}))
    }

    async fn soft_delete_article(
        &self,
        request: Request<SoftDeleteArticleRequest>,
    ) -> Result<Response<SoftDeleteArticleResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.inner()
            .soft_delete_article(command)
            .await
            .map_err(|why| {
                if why.is_domain_error() {
                    let why = why.as_domain_error().unwrap();

                    if !why.is_article_domain_error() {
                        return Status::internal("Internal Error");
                    }

                    let why = why.as_article_domain_error().unwrap();

                    if why.is_already_soft_deleted() {
                        return Status::failed_precondition(why.to_string());
                    }
                    return Status::internal("Internal Error");
                }

                if why.is_repository_error() {
                    let why = why.as_repository_error().unwrap();

                    if !why.is_article_repository_error() {
                        return Status::internal("Internal Error");
                    }

                    let why = why.as_article_repository_error().unwrap();

                    if why.is_article_not_found() {
                        return Status::not_found(why.to_string());
                    }

                    return Status::internal("Internal Error");
                }

                Status::internal("Internal Error")
            })?;

        Ok(Response::new(SoftDeleteArticleResponse {}))
    }

    async fn revoke_soft_delete_article(
        &self,
        request: Request<RevokeSoftDeleteArticleRequest>,
    ) -> Result<Response<RevokeSoftDeleteArticleResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.inner()
            .revoke_soft_delete_article(command)
            .await
            .map_err(|why| {
                if why.is_domain_error() {
                    let why = why.as_domain_error().unwrap();

                    if !why.is_article_domain_error() {
                        return Status::internal("Internal Error");
                    }

                    let why = why.as_article_domain_error().unwrap();

                    if why.is_not_soft_deleted() {
                        return Status::failed_precondition(why.to_string());
                    }
                    return Status::internal("Internal Error");
                }

                if why.is_repository_error() {
                    let why = why.as_repository_error().unwrap();

                    if !why.is_article_repository_error() {
                        return Status::internal("Internal Error");
                    }

                    let why = why.as_article_repository_error().unwrap();

                    if why.is_article_not_found() {
                        return Status::not_found(why.to_string());
                    }

                    return Status::internal("Internal Error");
                }

                Status::internal("Internal Error")
            })?;

        Ok(Response::new(RevokeSoftDeleteArticleResponse {}))
    }

    async fn delete_article(
        &self,
        request: Request<DeleteArticleRequest>,
    ) -> Result<Response<DeleteArticleResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.inner().delete_article(command).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_article_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_article_repository_error().unwrap();

            if why.is_article_not_found() {
                return Status::not_found(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        let reply = DeleteArticleResponse {};
        Ok(Response::new(reply))
    }

    async fn delete_series(
        &self,
        request: Request<DeleteSeriesRequest>,
    ) -> Result<Response<DeleteSeriesResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.inner().delete_series(command).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_series_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_series_repository_error().unwrap();

            if why.is_series_not_found() {
                return Status::not_found(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        Ok(Response::new(DeleteSeriesResponse {}))
    }

    async fn delete_category(
        &self,
        request: Request<DeleteCategoryRequest>,
    ) -> Result<Response<DeleteCategoryResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.inner().delete_category(command).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_category_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_category_repository_error().unwrap();

            if why.is_category_not_found() {
                return Status::not_found(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        Ok(Response::new(DeleteCategoryResponse {}))
    }

    async fn delete_tag(
        &self,
        request: Request<DeleteTagRequest>,
    ) -> Result<Response<DeleteTagResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.inner().delete_tag(command).await.map_err(|why| {
            if !why.is_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_repository_error().unwrap();

            if !why.is_tag_repository_error() {
                return Status::internal("Internal Error");
            }

            let why = why.as_tag_repository_error().unwrap();

            if why.is_tag_not_found() {
                return Status::not_found(why.to_string());
            }

            Status::internal("Internal Error")
        })?;

        Ok(Response::new(DeleteTagResponse {}))
    }
}
