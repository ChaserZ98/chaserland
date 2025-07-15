use chaserland_articles_service_core::{
    app::{
        command::{
            interface::ArticleCommandService as ArticleCommandServiceInterface,
            service::ArticleCommandService,
        },
        query::{
            interface::ArticleQueryService as ArticleQueryServiceInterface,
            service::ArticleQueryService,
        },
    },
    infra::postgres::{
        query_handler::{
            PgArticleQueryHandler, PgCategoryQueryHandler, PgSeriesQueryHandler, PgTagQueryHandler,
        },
        repository::{
            PgArticleRepository, PgCategoryRepository, PgSeriesRepository, PgTagRepository,
        },
    },
};
use sqlx::PgPool;

pub fn init_query_service(pool: PgPool) -> impl ArticleQueryServiceInterface {
    let article_query_handler = PgArticleQueryHandler::new(pool.clone());
    let series_query_handler = PgSeriesQueryHandler::new(pool.clone());
    let category_query_handler = PgCategoryQueryHandler::new(pool.clone());
    let tag_query_handler = PgTagQueryHandler::new(pool.clone());

    ArticleQueryService::new(
        article_query_handler,
        series_query_handler,
        category_query_handler,
        tag_query_handler,
    )
}

pub fn init_command_service(pool: PgPool) -> impl ArticleCommandServiceInterface {
    let article_repository = PgArticleRepository::new();
    let series_repository = PgSeriesRepository::new();
    let category_repository = PgCategoryRepository::new();
    let tag_repository = PgTagRepository::new();

    ArticleCommandService::new(
        article_repository,
        series_repository,
        category_repository,
        tag_repository,
        pool,
    )
}
