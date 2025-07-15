use crate::{
    app::{command::CreateTagCommand, query::dto::TagDTO},
    ports::rest::response::ErrorResponse,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct TagCreate {
    pub name: String,
}

impl TryInto<CreateTagCommand> for TagCreate {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<CreateTagCommand, Self::Error> {
        let name = self
            .name
            .try_into()
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;

        Ok(CreateTagCommand { name })
    }
}

#[derive(Serialize, ToSchema)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl From<TagDTO> for Tag {
    fn from(value: TagDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
        }
    }
}
