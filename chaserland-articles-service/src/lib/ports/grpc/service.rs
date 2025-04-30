use crate::app::service::ArticleService;
use crate::domain::repository::article::ArticleRepository;
use crate::domain::repository::category::CategoryRepository;
use crate::domain::repository::series::SeriesRepository;
use crate::domain::repository::tag::TagRepository;

use chaserland_protos::article::v1::{
    CreateArticleRequest, CreateArticleResponse, CreateCategoryRequest, CreateCategoryResponse,
    CreateSeriesRequest, CreateSeriesResponse, CreateTagRequest, CreateTagResponse,
    DeleteArticleByIdRequest, DeleteArticleByIdResponse, DeleteCategoryByIdRequest,
    DeleteCategoryByIdResponse, DeleteSeriesByIdRequest, DeleteSeriesByIdResponse,
    DeleteTagByIdRequest, DeleteTagByIdResponse, GetArticleContentRequest,
    GetArticleContentResponse, GetArticleRequest, GetArticleResponse, GetArticlesRequest,
    GetArticlesResponse, GetCategoriesRequest, GetCategoriesResponse, GetSeriesRequest,
    GetSeriesResponse, GetTagsRequest, GetTagsResponse, PublishArticleByIdRequest,
    PublishArticleByIdResponse, article_service_server::ArticleService as TonicArticleService,
};
use tonic::{Request, Response, Status};

pub struct GrpcArticleService<
    R: ArticleRepository,
    S: SeriesRepository,
    C: CategoryRepository,
    T: TagRepository,
>(ArticleService<R, S, C, T>);

#[tonic::async_trait]
impl<R: ArticleRepository, S: SeriesRepository, C: CategoryRepository, T: TagRepository>
    TonicArticleService for GrpcArticleService<R, S, C, T>
{
    async fn create_article(
        &self,
        request: Request<CreateArticleRequest>,
    ) -> Result<Response<CreateArticleResponse>, Status> {
        let message = request.into_inner();

        let article_create = message.try_into()?;

        let article = self.0.create_article(article_create).await.map_err(|e| {
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

        let reply = CreateArticleResponse {
            article: Some(article.into()),
        };
        Ok(Response::new(reply))
    }
    async fn create_series(
        &self,
        request: Request<CreateSeriesRequest>,
    ) -> Result<Response<CreateSeriesResponse>, Status> {
        let message = request.into_inner();

        let series_create = match message.series {
            Some(value) => value,
            None => return Err(Status::invalid_argument("Series is required")),
        };

        let series = match self.create_series(series_create).await {
            Ok(series) => series,
            Err(e) => {
                tracing::error!("Failed to create series: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };

        let reply = CreateSeriesResponse {
            series: Some(series.into()),
        };

        Ok(Response::new(reply))
    }
    async fn create_category(
        &self,
        request: Request<CreateCategoryRequest>,
    ) -> Result<Response<CreateCategoryResponse>, Status> {
        let message = request.into_inner();
        let category_create = match message.category {
            Some(value) => value,
            None => return Err(Status::invalid_argument("Category is required")),
        };

        let category = match self.create_category(category_create).await {
            Ok(category) => category,
            Err(e) => {
                tracing::error!("Failed to create category: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };

        let reply = CreateCategoryResponse {
            category: Some(category.into()),
        };
        Ok(Response::new(reply))
    }
    async fn create_tag(
        &self,
        request: Request<CreateTagRequest>,
    ) -> Result<Response<CreateTagResponse>, Status> {
        let message = request.into_inner();

        let tag_create = match message.tag {
            Some(value) => value,
            None => return Err(Status::invalid_argument("Tag is required")),
        };

        let tag = match self.create_tag(tag_create).await {
            Ok(tag) => tag,
            Err(e) => {
                tracing::error!("Failed to create tag: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };

        let reply = CreateTagResponse {
            tag: Some(tag.into()),
        };
        Ok(Response::new(reply))
    }
    async fn get_article(
        &self,
        request: Request<GetArticleRequest>,
    ) -> Result<Response<GetArticleResponse>, Status> {
        let message = request.into_inner();

        if message.identifier.is_none() {
            return Err(Status::invalid_argument(
                "Identifier 'id' or 'slug' is missing",
            ));
        }

        let article = match self
            .get_article(
                message.identifier.unwrap(),
                message.public_only,
                message.with_content,
            )
            .await
        {
            Ok(Some(article)) => article,
            Ok(None) => return Err(Status::not_found("Article not found")),
            Err(e) => {
                tracing::error!("Failed to get article: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };

        let reply = GetArticleResponse {
            article: Some(article.into()),
        };
        Ok(Response::new(reply))
    }
    async fn get_article_content(
        &self,
        request: Request<GetArticleContentRequest>,
    ) -> Result<Response<GetArticleContentResponse>, Status> {
        let message = request.into_inner();

        if message.identifier.is_none() {
            return Err(Status::invalid_argument("Identifier is required"));
        }

        let content = match self
            .get_article_content(message.identifier.unwrap(), message.public_only)
            .await
        {
            Ok(Some(content)) => content,
            Ok(None) => return Err(Status::not_found("Article not found")),
            Err(e) => {
                tracing::error!("Failed to get article content: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };

        let reply = GetArticleContentResponse { content };
        Ok(Response::new(reply))
    }
    async fn get_articles(
        &self,
        request: Request<GetArticlesRequest>,
    ) -> Result<Response<GetArticlesResponse>, Status> {
        let message = request.into_inner();
        if message.page < 1 {
            return Err(Status::invalid_argument(
                "Page must be greater or equal to 1",
            ));
        }

        if message.page_size < 1 {
            return Err(Status::invalid_argument(
                "Page size must be greater or equal to 1",
            ));
        }

        let articles_meta = match self
            .get_articles(
                message.page,
                message.page_size,
                message.public_only,
                message.with_content,
                message.filter,
            )
            .await
        {
            Ok(articles_meta) => articles_meta,
            Err(e) => {
                tracing::error!("Failed to get articles: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };

        let reply = GetArticlesResponse {
            articles: articles_meta.into_iter().map(|x| x.into()).collect(),
        };
        Ok(Response::new(reply))
    }
    async fn get_series(
        &self,
        _request: Request<GetSeriesRequest>,
    ) -> Result<Response<GetSeriesResponse>, Status> {
        let series = match self.get_series().await {
            Ok(series) => series,
            Err(e) => {
                tracing::error!("Failed to get series: {}", e);
                return Err(Status::internal("Failed to get series"));
            }
        };

        let reply = GetSeriesResponse {
            series: series.into_iter().map(|x| x.into()).collect(),
        };

        Ok(Response::new(reply))
    }
    async fn get_categories(
        &self,
        _request: Request<GetCategoriesRequest>,
    ) -> Result<Response<GetCategoriesResponse>, Status> {
        let categories = match self.get_categories().await {
            Ok(categories) => categories,
            Err(e) => {
                tracing::error!("Failed to get categories: {}", e);
                return Err(Status::internal("Failed to get categories"));
            }
        };

        let reply = GetCategoriesResponse {
            categories: categories.into_iter().map(|x| x.into()).collect(),
        };

        Ok(Response::new(reply))
    }
    async fn get_tags(
        &self,
        _request: Request<GetTagsRequest>,
    ) -> Result<Response<GetTagsResponse>, Status> {
        let tags = match self.get_tags().await {
            Ok(tags) => tags,
            Err(e) => {
                tracing::error!("Failed to get tags: {}", e);
                return Err(Status::internal("Failed to get tags"));
            }
        };

        let reply = GetTagsResponse {
            tags: tags.into_iter().map(|x| x.into()).collect(),
        };

        Ok(Response::new(reply))
    }
    async fn publish_article_by_id(
        &self,
        request: Request<PublishArticleByIdRequest>,
    ) -> Result<Response<PublishArticleByIdResponse>, Status> {
        let message = request.into_inner();

        match self.publish_article_by_id(message.id).await {
            Err(e) => {
                tracing::error!("Failed to publish article by id: {}", e);
                return Err(Status::internal("Internal Error"));
            }
            Ok(0) => {
                return Err(Status::not_found(format!(
                    "Article with id {} not found",
                    message.id
                )));
            }
            _ => {}
        }

        Ok(Response::new(PublishArticleByIdResponse {}))
    }
    async fn delete_article_by_id(
        &self,
        request: Request<DeleteArticleByIdRequest>,
    ) -> Result<Response<DeleteArticleByIdResponse>, Status> {
        let message = request.into_inner();

        match self.delete_article_by_id(message.id).await {
            Err(e) => {
                tracing::error!("Failed to delete article by id: {}", e);
                return Err(Status::internal("Internal Error"));
            }
            Ok(0) => {
                return Err(Status::not_found(format!(
                    "Article with id {} not found",
                    message.id
                )));
            }
            _ => {}
        }

        Ok(Response::new(DeleteArticleByIdResponse {}))
    }
    async fn delete_series_by_id(
        &self,
        request: Request<DeleteSeriesByIdRequest>,
    ) -> Result<Response<DeleteSeriesByIdResponse>, Status> {
        let message = request.into_inner();

        match self.delete_series_by_id(message.id).await {
            Err(e) => {
                tracing::error!("Failed to delete series by id: {}", e);
                return Err(Status::internal("Internal Error"));
            }
            Ok(0) => {
                return Err(Status::not_found(format!(
                    "Series with id {} not found",
                    message.id
                )));
            }
            _ => {}
        }

        Ok(Response::new(DeleteSeriesByIdResponse {}))
    }
    async fn delete_category_by_id(
        &self,
        request: Request<DeleteCategoryByIdRequest>,
    ) -> Result<Response<DeleteCategoryByIdResponse>, Status> {
        let message = request.into_inner();

        match self.delete_category_by_id(message.id).await {
            Err(e) => {
                tracing::error!("Failed to delete category by id: {}", e);
                return Err(Status::internal("Internal Error"));
            }
            Ok(0) => {
                return Err(Status::not_found(format!(
                    "Category with id {} not found",
                    message.id
                )));
            }
            _ => {}
        }

        Ok(Response::new(DeleteCategoryByIdResponse {}))
    }
    async fn delete_tag_by_id(
        &self,
        request: Request<DeleteTagByIdRequest>,
    ) -> Result<Response<DeleteTagByIdResponse>, Status> {
        let message = request.into_inner();

        match self.delete_tag_by_id(message.id).await {
            Err(e) => {
                tracing::error!("Failed to delete tag by id: {}", e);
                return Err(Status::internal("Internal Error"));
            }
            Ok(0) => {
                return Err(Status::not_found(format!(
                    "Tag with id {} not found",
                    message.id
                )));
            }
            _ => {}
        }

        Ok(Response::new(DeleteTagByIdResponse {}))
    }
}
