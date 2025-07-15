use crate::domain::series::vo as series;

#[derive(Debug)]
pub struct SeriesFilter {
    series_ids: Vec<series::Id>,
}

impl SeriesFilter {
    pub fn new(series_ids: Vec<series::Id>) -> Self {
        Self::validate(&series_ids).unwrap();
        Self { series_ids }
    }

    pub fn try_new(series_ids: Vec<series::Id>) -> Result<Self, String> {
        Self::validate(&series_ids)?;

        Ok(Self { series_ids })
    }

    pub fn series_ids(&self) -> &Vec<series::Id> {
        &self.series_ids
    }

    fn validate(series_ids: &Vec<series::Id>) -> Result<(), String> {
        match series_ids.is_empty() {
            true => Err("At least one series id must be specified".to_string()),
            false => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SeriesFilter;

    #[test]
    fn series_filter_case_new() {
        let series_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];

        let series_filter = SeriesFilter::new(series_ids.clone());

        assert_eq!(series_filter.series_ids(), &series_ids);
    }

    #[test]
    #[should_panic(expected = "At least one series id must be specified")]
    fn series_filter_case_new_panic() {
        SeriesFilter::new(vec![]);
    }

    #[test]
    fn series_filter_case_try_new() {
        let series_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];

        let res = SeriesFilter::try_new(series_ids.clone());

        assert!(res.is_ok());

        let filter = res.unwrap();

        assert_eq!(filter.series_ids(), &series_ids);

        let res = SeriesFilter::try_new(vec![]);

        assert!(res.is_err());

        assert_eq!(
            res.err().unwrap(),
            "At least one series id must be specified"
        );
    }
}
