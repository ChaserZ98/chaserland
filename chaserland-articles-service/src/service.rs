use crate::model;
use chaserland_protos::article::{
    CreateArticleRequest, CreateArticleResponse, CreateCategoryRequest, CreateCategoryResponse,
    CreateSeriesRequest, CreateSeriesResponse, CreateTagRequest, CreateTagResponse,
    DeleteArticleByIdRequest, DeleteArticleByIdResponse, DeleteCategoryByIdRequest,
    DeleteCategoryByIdResponse, DeleteSeriesByIdRequest, DeleteSeriesByIdResponse,
    DeleteTagByIdRequest, DeleteTagByIdResponse, GetArticleContentBySlugRequest,
    GetArticleContentBySlugResponse, GetArticlesMetaByCategorySlugRequest,
    GetArticlesMetaBySeriesSlugRequest, GetArticlesMetaByTagSlugRequest, GetArticlesMetaRequest,
    GetArticlesMetaResponse, GetCategoriesRequest, GetCategoriesResponse, GetSeriesRequest,
    GetSeriesResponse, GetTagsRequest, GetTagsResponse, PublishArticleByIdRequest,
    PublishArticleByIdResponse,
    article_service_server::{ArticleService as TonicArticleService, ArticleServiceServer},
};
use sqlx::PgPool;
use tonic::{Request, Response, Status};

pub struct ArticleService {
    db: PgPool,
}

impl ArticleService {
    pub fn new(db: PgPool) -> ArticleServiceServer<Self> {
        ArticleServiceServer::new(Self { db })
    }
}

#[tonic::async_trait]
impl TonicArticleService for ArticleService {
    async fn create_article(
        &self,
        request: Request<CreateArticleRequest>,
    ) -> Result<Response<CreateArticleResponse>, Status> {
        let message = request.get_ref();

        let article_create = match message.article.clone() {
            Some(value) => value,
            None => return Err(Status::invalid_argument("Article is required")),
        };

        let series_id = article_create.series_id.clone();
        let category_ids = article_create.category_ids.clone();
        let tag_ids = article_create.tag_ids.clone();

        let mut transaction = self.db.begin().await.map_err(|why| {
            tracing::error!("Failed to start transaction: {}", why);
            Status::internal("Internal Error")
        })?;

        let article = model::Article::create(&mut transaction, article_create.clone())
            .await
            .map_err(|why| {
                tracing::error!("Failed to create article: {}", why);
                Status::internal("Internal Error")
            })?;

        let series = match series_id {
            Some(series_id) => Some(
                model::Article::update_series(&mut transaction, article.id, series_id)
                    .await
                    .map_err(|why| {
                        tracing::error!("Failed to update series: {}", why);
                        Status::internal("Internal Error")
                    })?,
            ),
            None => None,
        };

        let categories = match category_ids.is_empty() {
            true => vec![],
            false => {
                model::Article::add_categories(&mut transaction, article.id, category_ids.clone())
                    .await
                    .map_err(|why| {
                        tracing::error!("Failed to add categories: {}", why);
                        Status::internal("Internal Error")
                    })?
            }
        };

        let tags = match tag_ids.is_empty() {
            true => vec![],
            false => model::Article::add_tags(&mut transaction, article.id, tag_ids.clone())
                .await
                .map_err(|why| {
                    tracing::error!("Failed to add tags: {}", why);
                    Status::internal("Internal Error")
                })?,
        };

        transaction.commit().await.map_err(|why| {
            tracing::error!("Failed to commit transaction: {}", why);
            Status::internal("Internal Error")
        })?;

        let full_article = model::FullArticle {
            id: article.id,
            title: article.title,
            slug: article.slug,
            description: article.description,
            content: article.content,
            created_at: article.created_at,
            published_at: article.published_at,
            updated_at: article.updated_at,
            deleted_at: article.deleted_at,
            series,
            categories,
            tags,
        };

        let reply = CreateArticleResponse {
            article: Some(full_article.into()),
        };
        Ok(Response::new(reply))
    }
    async fn create_series(
        &self,
        request: Request<CreateSeriesRequest>,
    ) -> Result<Response<CreateSeriesResponse>, Status> {
        let message = request.get_ref();
        let series_create = message.series.clone();
        if series_create.is_none() {
            return Err(Status::invalid_argument("Series is required"));
        }
        let series_create = series_create.unwrap();
        let mut transaction = self.db.begin().await.map_err(|why| {
            tracing::error!("Failed to start transaction: {}", why);
            Status::internal("Internal Error")
        })?;

        let series = model::Series::create(&mut transaction, series_create)
            .await
            .map_err(|why| {
                tracing::error!("Failed to create series: {}", why);
                Status::internal("Internal Error")
            })?;

        transaction.commit().await.map_err(|why| {
            tracing::error!("Failed to commit transaction: {}", why);
            Status::internal("Internal Error")
        })?;

        let reply = CreateSeriesResponse {
            series: Some(series.into()),
        };

        Ok(Response::new(reply))
    }
    async fn create_category(
        &self,
        request: Request<CreateCategoryRequest>,
    ) -> Result<Response<CreateCategoryResponse>, Status> {
        let message = request.get_ref();

        let category_create = match message.category.clone() {
            Some(value) => value,
            None => {
                return Err(Status::invalid_argument("Category is required"));
            }
        };

        let mut transaction = match self.db.begin().await {
            Ok(transaction) => transaction,
            Err(e) => {
                tracing::error!("Failed to start transaction: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };

        let category = match model::Category::create(&mut transaction, category_create).await {
            Ok(category) => category,
            Err(e) => {
                tracing::error!("Failed to create category: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };

        transaction.commit().await.map_err(|why| {
            tracing::error!("Failed to commit transaction: {}", why);
            Status::internal("Internal Error")
        })?;

        let reply = CreateCategoryResponse {
            category: Some(category.into()),
        };
        Ok(Response::new(reply))
    }
    async fn create_tag(
        &self,
        request: Request<CreateTagRequest>,
    ) -> Result<Response<CreateTagResponse>, Status> {
        let message = request.get_ref();

        let tag_create = match message.tag.clone() {
            Some(value) => value,
            None => {
                return Err(Status::invalid_argument("Tag is required"));
            }
        };

        let mut transaction = match self.db.begin().await {
            Ok(transaction) => transaction,
            Err(e) => {
                tracing::error!("Failed to start transaction: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };

        let tag = model::Tag::create(&mut transaction, tag_create)
            .await
            .map_err(|why| {
                tracing::error!("Failed to create tag: {}", why);
                Status::internal("Internal Error")
            })?;

        transaction.commit().await.map_err(|why| {
            tracing::error!("Failed to commit transaction: {}", why);
            Status::internal("Internal Error")
        })?;

        let reply = CreateTagResponse {
            tag: Some(tag.into()),
        };
        Ok(Response::new(reply))
    }
    async fn get_articles_meta(
        &self,
        request: Request<GetArticlesMetaRequest>,
    ) -> Result<Response<GetArticlesMetaResponse>, Status> {
        let message = request.get_ref();
        let page = message.page;
        let page_size = message.page_size;
        if page < 1 {
            return Err(Status::invalid_argument(
                "Page must be greater or equal to 1",
            ));
        }

        if page_size < 1 {
            return Err(Status::invalid_argument(
                "Page size must be greater or equal to 1",
            ));
        }

        let articles_meta = match model::FullArticleMeta::get(&self.db, page, page_size).await {
            Ok(articles_meta) => articles_meta,
            Err(e) => {
                tracing::error!("Failed to get articles meta: {}", e);
                return Err(Status::internal("Failed to get articles meta"));
            }
        };

        let reply = GetArticlesMetaResponse {
            articles_metas: articles_meta.into_iter().map(|x| x.into()).collect(),
        };
        Ok(Response::new(reply))
    }
    async fn get_articles_meta_by_series_slug(
        &self,
        request: Request<GetArticlesMetaBySeriesSlugRequest>,
    ) -> Result<Response<GetArticlesMetaResponse>, Status> {
        let message = request.get_ref();

        let series_slug = message.series_slug.clone();

        let articles_meta =
            match model::FullArticleMeta::get_by_series_slug(&self.db, series_slug).await {
                Ok(articles_meta) => articles_meta,
                Err(e) => {
                    tracing::error!("Failed to get articles meta by series slug: {}", e);
                    return Err(Status::internal(
                        "Failed to get articles meta by series slug",
                    ));
                }
            };

        let reply = GetArticlesMetaResponse {
            articles_metas: articles_meta.into_iter().map(|x| x.into()).collect(),
        };
        Ok(Response::new(reply))
    }
    async fn get_articles_meta_by_category_slug(
        &self,
        request: Request<GetArticlesMetaByCategorySlugRequest>,
    ) -> Result<Response<GetArticlesMetaResponse>, Status> {
        let message = request.get_ref();

        let category_slug = message.category_slug.clone();

        let page = message.page;
        let page_size = message.page_size;
        if page < 1 {
            return Err(Status::invalid_argument(
                "Page must be greater or equal to 1",
            ));
        }

        if page_size < 1 {
            return Err(Status::invalid_argument(
                "Page size must be greater or equal to 1",
            ));
        }

        let articles_meta = match model::FullArticleMeta::get_by_category_slug(
            &self.db,
            category_slug,
            page,
            page_size,
        )
        .await
        {
            Ok(articles_meta) => articles_meta,
            Err(e) => {
                tracing::error!("Failed to get articles meta by category slug: {}", e);
                return Err(Status::internal(
                    "Failed to get articles meta by category slug",
                ));
            }
        };

        let reply = GetArticlesMetaResponse {
            articles_metas: articles_meta.into_iter().map(|x| x.into()).collect(),
        };
        Ok(Response::new(reply))
    }
    async fn get_articles_meta_by_tag_slug(
        &self,
        request: Request<GetArticlesMetaByTagSlugRequest>,
    ) -> Result<Response<GetArticlesMetaResponse>, Status> {
        let message = request.get_ref();

        let tag_slug = message.tag_slug.clone();

        let page = message.page;
        let page_size = message.page_size;
        if page < 1 {
            return Err(Status::invalid_argument(
                "Page must be greater or equal to 1",
            ));
        }

        if page_size < 1 {
            return Err(Status::invalid_argument(
                "Page size must be greater or equal to 1",
            ));
        }

        let articles_meta = match model::FullArticleMeta::get_by_tag_slug(
            &self.db, tag_slug, page, page_size,
        )
        .await
        {
            Ok(articles_meta) => articles_meta,
            Err(e) => {
                tracing::error!("Failed to get articles meta by tag slug: {}", e);
                return Err(Status::internal("Failed to get articles meta by tag slug"));
            }
        };

        let reply = GetArticlesMetaResponse {
            articles_metas: articles_meta.into_iter().map(|x| x.into()).collect(),
        };
        Ok(Response::new(reply))
    }
    async fn get_article_content_by_slug(
        &self,
        request: Request<GetArticleContentBySlugRequest>,
    ) -> Result<Response<GetArticleContentBySlugResponse>, Status> {
        let message = request.get_ref();

        let slug = message.slug.clone();

        let content = match model::Article::get_content_by_slug(&self.db, slug).await {
            Ok(content) => content,
            Err(e) => {
                tracing::error!("Failed to get article content by slug: {}", e);
                return Err(Status::internal("Failed to get article content by slug"));
            }
        };

        if content.is_none() {
            return Err(Status::not_found("Article not found"));
        }

        let content = content.unwrap();

        let reply = GetArticleContentBySlugResponse { content };
        Ok(Response::new(reply))
    }
    async fn get_series(
        &self,
        _request: Request<GetSeriesRequest>,
    ) -> Result<Response<GetSeriesResponse>, Status> {
        let series = match model::Series::get(&self.db).await {
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
        let categories = match model::Category::get(&self.db).await {
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
        let tags = match model::Tag::get(&self.db).await {
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
        let message = request.get_ref();

        let id = message.id;

        let mut transaction = match self.db.begin().await {
            Ok(transaction) => transaction,
            Err(e) => {
                tracing::error!("Failed to start transaction: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };

        match model::Article::publish_by_id(&mut transaction, id).await {
            Err(e) => {
                tracing::error!("Failed to publish article by id: {}", e);
                return Err(Status::internal("Internal Error"));
            }
            Ok(0) => {
                return Err(Status::not_found("Article not found"));
            }
            _ => {}
        };

        transaction.commit().await.map_err(|why| {
            tracing::error!("Failed to commit transaction: {}", why);
            Status::internal("Internal Error")
        })?;

        Ok(Response::new(PublishArticleByIdResponse {}))
    }
    async fn delete_article_by_id(
        &self,
        request: Request<DeleteArticleByIdRequest>,
    ) -> Result<Response<DeleteArticleByIdResponse>, Status> {
        let message = request.get_ref();

        let id = message.id;

        let mut transaction = match self.db.begin().await {
            Ok(transaction) => transaction,
            Err(e) => {
                tracing::error!("Failed to start transaction: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };

        match model::Article::delete_by_id(&mut transaction, id).await {
            Err(e) => {
                tracing::error!("Failed to delete article by id: {}", e);
                return Err(Status::internal("Internal Error"));
            }
            Ok(0) => {
                return Err(Status::not_found(format!(
                    "Article with id {} not found",
                    id
                )));
            }
            _ => {}
        };

        transaction.commit().await.map_err(|why| {
            tracing::error!("Failed to commit transaction: {}", why);
            Status::internal("Internal Error")
        })?;

        Ok(Response::new(DeleteArticleByIdResponse {}))
    }
    async fn delete_series_by_id(
        &self,
        request: Request<DeleteSeriesByIdRequest>,
    ) -> Result<Response<DeleteSeriesByIdResponse>, Status> {
        let message = request.get_ref();
        let id = message.id;
        let mut transaction = match self.db.begin().await {
            Ok(transaction) => transaction,
            Err(e) => {
                tracing::error!("Failed to start transaction: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };
        match model::Series::delete_by_id(&mut transaction, id).await {
            Err(e) => {
                tracing::error!("Failed to delete series by id: {}", e);
                return Err(Status::internal("Internal Error"));
            }
            Ok(0) => {
                return Err(Status::not_found(format!(
                    "Series with id {} not found",
                    id
                )));
            }
            _ => {}
        };

        transaction.commit().await.map_err(|why| {
            tracing::error!("Failed to commit transaction: {}", why);
            Status::internal("Internal Error")
        })?;

        Ok(Response::new(DeleteSeriesByIdResponse {}))
    }
    async fn delete_category_by_id(
        &self,
        request: Request<DeleteCategoryByIdRequest>,
    ) -> Result<Response<DeleteCategoryByIdResponse>, Status> {
        let message = request.get_ref();

        let id = message.id;
        let mut transaction = match self.db.begin().await {
            Ok(transaction) => transaction,
            Err(e) => {
                tracing::error!("Failed to start transaction: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };
        match model::Category::delete_by_id(&mut transaction, id).await {
            Err(e) => {
                tracing::error!("Failed to delete category by id: {}", e);
                return Err(Status::internal("Internal Error"));
            }
            Ok(0) => {
                return Err(Status::not_found(format!(
                    "Category with id {} not found",
                    id
                )));
            }
            _ => {}
        };

        transaction.commit().await.map_err(|why| {
            tracing::error!("Failed to commit transaction: {}", why);
            Status::internal("Internal Error")
        })?;

        Ok(Response::new(DeleteCategoryByIdResponse {}))
    }
    async fn delete_tag_by_id(
        &self,
        request: Request<DeleteTagByIdRequest>,
    ) -> Result<Response<DeleteTagByIdResponse>, Status> {
        let message = request.get_ref();

        let id = message.id;

        let mut transaction = match self.db.begin().await {
            Ok(transaction) => transaction,
            Err(e) => {
                tracing::error!("Failed to start transaction: {}", e);
                return Err(Status::internal("Internal Error"));
            }
        };
        match model::Tag::delete_by_id(&mut transaction, id).await {
            Err(e) => {
                tracing::error!("Failed to delete tag by id: {}", e);
                return Err(Status::internal("Internal Error"));
            }
            Ok(0) => {
                return Err(Status::not_found(format!("Tag with id {} not found", id)));
            }
            _ => {}
        };

        transaction.commit().await.map_err(|why| {
            tracing::error!("Failed to commit transaction: {}", why);
            Status::internal("Internal Error")
        })?;

        Ok(Response::new(DeleteTagByIdResponse {}))
    }
}
