use chaserland_articles_service_core::{
    app::{
        command::{self as command, interface::ArticleCommandService},
        query::{self as query, dto::TagDTO, interface::ArticleQueryService},
    },
    domain::tag::vo as tag,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

use crate::common::{init_command_service, init_query_service};

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../tests/fixtures",
        scripts(
            "series",
            "categories",
            "tags",
            "articles",
            "article_categories",
            "article_tags"
        )
    )
)]
async fn create_tag_case_1(pool: PgPool) {
    let command_service = init_command_service(pool.clone());
    let query_service = init_query_service(pool);

    let name = "Tag 4".try_into().unwrap();
    let command = command::CreateTagCommand { name };
    let res = command_service.create_tag(command).await;

    assert!(res.is_ok());

    let query = query::GetTagOneQuery {
        identifier: tag::Id::new(4).as_identifier(),
    };
    let res = query_service.get_tag_one(query).await;
    let target = TagDTO {
        id: 4,
        name: "Tag 4".into(),
        slug: "tag-4".into(),
    };

    assert!(res.is_ok());

    let res = res.unwrap();
    assert_eq!(res, target);
}
