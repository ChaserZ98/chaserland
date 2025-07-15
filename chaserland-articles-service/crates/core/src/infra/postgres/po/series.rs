use crate::{app::query::dto::SeriesDTO, domain::series::entity::Series};

#[derive(sqlx::FromRow)]
pub struct PgSeries {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl TryInto<Series> for PgSeries {
    type Error = String;

    fn try_into(self) -> Result<Series, Self::Error> {
        let id = self.id.try_into()?;
        let name = self.name.try_into()?;

        let series = Series::new(id, name);

        Ok(series)
    }
}

impl Into<SeriesDTO> for PgSeries {
    fn into(self) -> SeriesDTO {
        SeriesDTO {
            id: self.id,
            name: self.name,
            slug: self.slug,
        }
    }
}
