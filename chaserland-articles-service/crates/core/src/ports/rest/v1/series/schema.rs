use crate::{
    app::{command::CreateSeriesCommand, query::dto::SeriesDTO},
    ports::rest::response::ErrorResponse,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct SeriesCreate {
    pub name: String,
}

impl TryInto<CreateSeriesCommand> for SeriesCreate {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<CreateSeriesCommand, Self::Error> {
        let name = self
            .name
            .try_into()
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;

        Ok(CreateSeriesCommand { name })
    }
}

#[derive(Serialize, ToSchema)]
pub struct Series {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl From<SeriesDTO> for Series {
    fn from(value: SeriesDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
        }
    }
}
