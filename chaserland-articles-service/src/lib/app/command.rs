use crate::domain::article::vo as article;
use crate::domain::category::vo as category;
use crate::domain::series::vo as series;
use crate::domain::tag::vo as tag;

pub struct CreateArticleCommand {
    pub title: article::Title,
    pub description: article::Description,
    pub content: Option<article::Content>,
    pub series_id: Option<series::Id>,
    pub category_ids: Vec<category::Id>,
    pub tag_ids: Vec<tag::Id>,
}

pub struct CreateSeriesCommand {
    pub name: series::Name,
}

pub struct CreateCategoryCommand {
    pub name: category::Name,
}

pub struct CreateTagCommand {
    pub name: tag::Name,
}

pub struct PublishArticleCommand {
    pub identifier: article::Identifier,
}

pub struct UnpublishArticleCommand {
    pub identifier: article::Identifier,
}

pub struct SoftDeleteArticleCommand {
    pub identifier: article::Identifier,
}

pub struct RevokeSoftDeleteArticleCommand {
    pub identifier: article::Identifier,
}

pub struct DeleteArticleCommand {
    pub identifier: article::Identifier,
}

pub struct DeleteSeriesCommand {
    pub identifier: series::Identifier,
}

pub struct DeleteCategoryCommand {
    pub identifier: category::Identifier,
}

pub struct DeleteTagCommand {
    pub identifier: tag::Identifier,
}
