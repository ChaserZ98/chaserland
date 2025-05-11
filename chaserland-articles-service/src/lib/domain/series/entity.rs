use super::vo::{Id, Name, Slug};
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

#[cfg(test)]
mod tests {
    use super::Series;

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
