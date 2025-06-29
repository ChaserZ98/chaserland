use crate::{
    app::{command::CreateCategoryCommand, dto::CategoryDTO},
    ports::rest::response::ErrorResponse,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CategoryCreate {
    pub name: String,
}

impl TryInto<CreateCategoryCommand> for CategoryCreate {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<CreateCategoryCommand, Self::Error> {
        let name = self
            .name
            .try_into()
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;

        Ok(CreateCategoryCommand { name })
    }
}

#[derive(Serialize, ToSchema)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl From<CategoryDTO> for Category {
    fn from(value: CategoryDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
        }
    }
}
