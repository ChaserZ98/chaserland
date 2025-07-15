use chaserland_articles_service_core::{
    app::query::{
        dto::TagDTO,
        query_handler::{error::TagQueryHandlerError, interface::TagQueryHandler},
    },
    domain::tag::vo as tag,
    infra::postgres::query_handler::PgTagQueryHandler,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_one_case_id(pool: PgPool) {
    let query_handler = PgTagQueryHandler::new(pool);

    let target = TagDTO {
        id: 1,
        name: "Tag 1".into(),
        slug: "tag-1".into(),
    };

    let res = query_handler
        .get_one(tag::Id::new(target.id).as_identifier())
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_one_case_slug(pool: PgPool) {
    let query_handler = PgTagQueryHandler::new(pool);

    let target = TagDTO {
        id: 1,
        name: "Tag 1".into(),
        slug: "tag-1".into(),
    };

    let res = query_handler
        .get_one(
            tag::Name::new(target.name.clone())
                .as_slug()
                .as_identifier(),
        )
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_one_case_id_not_found(pool: PgPool) {
    let query_handler = PgTagQueryHandler::new(pool);

    let identifier = tag::Id::new(4).as_identifier();

    let res = query_handler.get_one(identifier.clone()).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        TagQueryHandlerError::TagNotFound(value) => value == identifier,
        _ => false,
    });
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_one_case_slug_not_found(pool: PgPool) {
    let query_handler = PgTagQueryHandler::new(pool);

    let identifier = tag::Name::new("tag 4").as_slug().as_identifier();

    let res = query_handler.get_one(identifier.clone()).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        TagQueryHandlerError::TagNotFound(value) => value == identifier,
        _ => false,
    });
}
