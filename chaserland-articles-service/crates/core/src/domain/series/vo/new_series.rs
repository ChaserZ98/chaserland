use super::{Name, Slug};

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
    use super::NewSeries;

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
