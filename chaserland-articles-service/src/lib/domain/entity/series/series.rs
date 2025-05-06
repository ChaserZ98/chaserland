use super::{Id, Name, Slug};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Series {
    pub id: Id,
    slug: Slug,
    pub name: Name,
}

impl Series {
    pub fn new(id: Id, name: Name) -> Self {
        let slug = name.as_slug();
        Self { id, slug, name }
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }
}

impl Default for Series {
    fn default() -> Self {
        let id = Id::new(1);
        let name = Name::new("default");
        Self::new(id, name)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewSeries {
    pub name: Name,
    slug: Slug,
}

impl NewSeries {
    pub fn new(name: Name) -> Self {
        let slug = name.as_slug();
        Self { name, slug }
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }
}

#[cfg(test)]
mod tests {
    mod series {
        use super::super::Series;

        #[test]
        fn series_case_new() {
            let series = Series::new(1.try_into().unwrap(), "series 1".try_into().unwrap());

            assert_eq!(series.id, 1.try_into().unwrap());
            assert_eq!(series.slug, "series-1".try_into().unwrap());
            assert_eq!(series.name, "series 1".try_into().unwrap());
        }

        #[test]
        fn series_case_default() {
            let series = Series::default();

            assert_eq!(series.id, 1.try_into().unwrap());
            assert_eq!(series.slug, "default".try_into().unwrap());
            assert_eq!(series.name, "default".try_into().unwrap());
        }

        #[test]
        fn series_case_slug() {
            let series = Series::new(1.try_into().unwrap(), "series 1".try_into().unwrap());
            let slug = "series-1".try_into().unwrap();
            assert_eq!(series.slug(), &slug);
        }
    }

    mod new_series {
        use super::super::NewSeries;

        #[test]
        fn new_series_case_new() {
            let new_series = NewSeries::new("series 1".try_into().unwrap());

            assert_eq!(new_series.name, "series 1".try_into().unwrap());
            assert_eq!(new_series.slug, "series-1".try_into().unwrap());
        }

        #[test]
        fn new_series_case_slug() {
            let new_series = NewSeries::new("series 1".try_into().unwrap());
            let slug = "series-1".try_into().unwrap();
            assert_eq!(new_series.slug(), &slug);
        }
    }
}
