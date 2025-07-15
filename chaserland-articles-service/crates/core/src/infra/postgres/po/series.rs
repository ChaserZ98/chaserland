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

#[cfg(test)]
mod tests {
    use super::PgSeries;
    use crate::{app::query::dto::SeriesDTO, domain::series::entity::Series};

    #[test]
    fn pg_series_try_into_series() {
        let pg_series = PgSeries {
            id: 1,
            name: "name".to_string(),
            slug: "name".to_string(),
        };

        let target = Series::new(1.try_into().unwrap(), "name".try_into().unwrap());

        let res: Result<Series, _> = pg_series.try_into();

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res, target);
    }

    #[test]
    fn pg_series_into_series_dto() {
        let pg_series = PgSeries {
            id: 1,
            name: "name".to_string(),
            slug: "name".to_string(),
        };

        let target = SeriesDTO {
            id: 1,
            name: "name".to_string(),
            slug: "name".to_string(),
        };

        let res: SeriesDTO = pg_series.into();

        assert_eq!(res, target);
    }
}
