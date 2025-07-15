use crate::{
    app::{
        command::{
            interface::ArticleCommandService as ArticleCommandServiceInterface,
            service::ArticleCommandService,
        },
        query::{
            interface::ArticleQueryService as ArticleQueryServiceInterface,
            query_handler::interface::{
                ArticleQueryHandler, CategoryQueryHandler, SeriesQueryHandler, TagQueryHandler,
            },
            service::ArticleQueryService,
        },
    },
    domain::{
        article::repository::ArticleRepository, category::repository::CategoryRepository,
        series::repository::SeriesRepository, tag::repository::TagRepository,
    },
};
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
use sqlx::Database;
use tonic::{Request, Response, Status};

pub struct GrpcArticleService<
    DB: Database,
    R: ArticleRepository<DB = DB>,
    S: SeriesRepository<DB = DB>,
    C: CategoryRepository<DB = DB>,
    T: TagRepository<DB = DB>,
    AQ: ArticleQueryHandler,
    SQ: SeriesQueryHandler,
    CQ: CategoryQueryHandler,
    TQ: TagQueryHandler,
> {
    command_service: ArticleCommandService<R, S, C, T, DB>,
    query_service: ArticleQueryService<AQ, SQ, CQ, TQ>,
}

// impl<R, S, C, T> Into<GrpcArticleService<R, S, C, T, A>> for ArticleCommandService<R, S, C, T>
// where
//     R: ArticleRepository,
//     S: SeriesRepository,
//     C: CategoryRepository,
//     T: TagRepository,
// {
//     fn into(self) -> GrpcArticleService<R, S, C, T> {
//         GrpcArticleService::new(self)
//     }
// }

impl<DB, R, S, C, T, AQ, SQ, CQ, TQ> GrpcArticleService<DB, R, S, C, T, AQ, SQ, CQ, TQ>
where
    DB: Database,
    R: ArticleRepository<DB = DB>,
    S: SeriesRepository<DB = DB>,
    C: CategoryRepository<DB = DB>,
    T: TagRepository<DB = DB>,
    AQ: ArticleQueryHandler,
    SQ: SeriesQueryHandler,
    CQ: CategoryQueryHandler,
    TQ: TagQueryHandler,
{
    pub fn new(
        command_service: ArticleCommandService<R, S, C, T, DB>,
        query_service: ArticleQueryService<AQ, SQ, CQ, TQ>,
    ) -> Self {
        Self {
            command_service,
            query_service,
        }
    }

    pub fn into_tonic_service(self) -> ArticleServiceServer<Self> {
        ArticleServiceServer::new(self)
    }
}

#[tonic::async_trait]
impl<DB, R, S, C, T, AQ, SQ, CQ, TQ> TonicArticleService
    for GrpcArticleService<DB, R, S, C, T, AQ, SQ, CQ, TQ>
where
    DB: Database,
    R: ArticleRepository<DB = DB>,
    S: SeriesRepository<DB = DB>,
    C: CategoryRepository<DB = DB>,
    T: TagRepository<DB = DB>,
    AQ: ArticleQueryHandler,
    SQ: SeriesQueryHandler,
    CQ: CategoryQueryHandler,
    TQ: TagQueryHandler,
{
    async fn create_article(
        &self,
        request: Request<CreateArticleRequest>,
    ) -> Result<Response<CreateArticleResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.command_service.create_article(command).await?;

        let reply = CreateArticleResponse {};
        Ok(Response::new(reply))
    }

    async fn create_series(
        &self,
        request: Request<CreateSeriesRequest>,
    ) -> Result<Response<CreateSeriesResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.command_service.create_series(command).await?;

        let reply = CreateSeriesResponse {};

        Ok(Response::new(reply))
    }

    async fn create_category(
        &self,
        request: Request<CreateCategoryRequest>,
    ) -> Result<Response<CreateCategoryResponse>, Status> {
        let message = request.into_inner();
        let command = message.try_into()?;

        self.command_service.create_category(command).await?;

        let reply = CreateCategoryResponse {};
        Ok(Response::new(reply))
    }

    async fn create_tag(
        &self,
        request: Request<CreateTagRequest>,
    ) -> Result<Response<CreateTagResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.command_service.create_tag(command).await?;

        let reply = CreateTagResponse {};
        Ok(Response::new(reply))
    }

    async fn get_article_one(
        &self,
        request: Request<GetArticleOneRequest>,
    ) -> Result<Response<GetArticleOneResponse>, Status> {
        let message = request.into_inner();
        let query = message.try_into()?;

        let article = self.query_service.get_article_one(query).await?;

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

        let content = self.query_service.get_article_content(query).await?;

        let reply = GetArticleContentResponse { content };
        Ok(Response::new(reply))
    }

    async fn get_article_many(
        &self,
        request: Request<GetArticleManyRequest>,
    ) -> Result<Response<GetArticleManyResponse>, Status> {
        let message = request.into_inner();
        let query = message.try_into()?;

        let articles = self.query_service.get_article_many(query).await?;

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

        let series = self.query_service.get_series_one(query).await?;

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

        let series = self.query_service.get_series_many(query).await?;

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

        let category = self.query_service.get_category_one(query).await?;

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

        let categories = self.query_service.get_category_many(query).await?;

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

        let tag = self.query_service.get_tag_one(query).await?;

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

        let tags = self.query_service.get_tag_many(query).await?;

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

        self.command_service.publish_article(command).await?;

        Ok(Response::new(PublishArticleResponse {}))
    }

    async fn unpublish_article(
        &self,
        request: Request<UnpublishArticleRequest>,
    ) -> Result<Response<UnpublishArticleResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.command_service.unpublish_article(command).await?;

        Ok(Response::new(UnpublishArticleResponse {}))
    }

    async fn soft_delete_article(
        &self,
        request: Request<SoftDeleteArticleRequest>,
    ) -> Result<Response<SoftDeleteArticleResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.command_service.soft_delete_article(command).await?;

        Ok(Response::new(SoftDeleteArticleResponse {}))
    }

    async fn revoke_soft_delete_article(
        &self,
        request: Request<RevokeSoftDeleteArticleRequest>,
    ) -> Result<Response<RevokeSoftDeleteArticleResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.command_service
            .revoke_soft_delete_article(command)
            .await?;

        Ok(Response::new(RevokeSoftDeleteArticleResponse {}))
    }

    async fn delete_article(
        &self,
        request: Request<DeleteArticleRequest>,
    ) -> Result<Response<DeleteArticleResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.command_service.delete_article(command).await?;

        let reply = DeleteArticleResponse {};
        Ok(Response::new(reply))
    }

    async fn delete_series(
        &self,
        request: Request<DeleteSeriesRequest>,
    ) -> Result<Response<DeleteSeriesResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.command_service.delete_series(command).await?;

        Ok(Response::new(DeleteSeriesResponse {}))
    }

    async fn delete_category(
        &self,
        request: Request<DeleteCategoryRequest>,
    ) -> Result<Response<DeleteCategoryResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.command_service.delete_category(command).await?;

        Ok(Response::new(DeleteCategoryResponse {}))
    }

    async fn delete_tag(
        &self,
        request: Request<DeleteTagRequest>,
    ) -> Result<Response<DeleteTagResponse>, Status> {
        let message = request.into_inner();

        let command = message.try_into()?;

        self.command_service.delete_tag(command).await?;

        Ok(Response::new(DeleteTagResponse {}))
    }
}
