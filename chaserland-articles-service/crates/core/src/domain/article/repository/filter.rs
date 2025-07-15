use crate::domain::category::vo as category;
use crate::domain::series::vo as series;
use crate::domain::tag::vo as tag;

/**
Note: A valid filter should at least have one of the following:
    - Non-empty series_identifier
    - Non-empty category_ids
    - Non-empty tag_ids

if category_ids and tag_ids is empty, it will be ignored or meaning matching all case.
*/
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArticlesFilter {
    series_identifier: Option<series::Identifier>,
    category_ids: Vec<category::Id>,
    tag_ids: Vec<tag::Id>,
}

impl ArticlesFilter {
    pub fn new(
        series_identifier: Option<series::Identifier>,
        category_ids: Vec<category::Id>,
        tag_ids: Vec<tag::Id>,
    ) -> Self {
        Self::validate(&series_identifier, &category_ids, &tag_ids).unwrap();
        Self {
            series_identifier,
            category_ids,
            tag_ids,
        }
    }

    pub fn try_new(
        series_identifier: Option<series::Identifier>,
        category_ids: Vec<category::Id>,
        tag_ids: Vec<tag::Id>,
    ) -> Result<Self, String> {
        Self::validate(&series_identifier, &category_ids, &tag_ids)?;

        let res = Self {
            series_identifier,
            category_ids,
            tag_ids,
        };
        Ok(res)
    }

    pub fn series_identifier(&self) -> &Option<series::Identifier> {
        &self.series_identifier
    }

    pub fn category_ids(&self) -> &Vec<category::Id> {
        &self.category_ids
    }

    pub fn tag_ids(&self) -> &Vec<tag::Id> {
        &self.tag_ids
    }

    fn validate(
        series_identifier: &Option<series::Identifier>,
        category_ids: &Vec<category::Id>,
        tag_ids: &Vec<tag::Id>,
    ) -> Result<(), String> {
        if series_identifier.is_none() && category_ids.is_empty() && tag_ids.is_empty() {
            return Err("At least one filter must be specified".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ArticlesFilter;
    use crate::domain::series::vo as series;

    #[test]
    fn articles_filter_case_new() {
        let series_id = series::Id::new(1);
        let category_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];
        let tag_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];

        let filter = ArticlesFilter::new(
            Some(series_id.as_identifier()),
            category_ids.clone(),
            tag_ids.clone(),
        );

        assert_eq!(filter.series_identifier(), &Some(series_id.as_identifier()));
        assert_eq!(filter.category_ids(), &category_ids);
        assert_eq!(filter.tag_ids(), &tag_ids);
    }

    #[test]
    #[should_panic(expected = "At least one filter must be specified")]
    fn articles_filter_case_new_panic() {
        ArticlesFilter::new(None, vec![], vec![]);
    }

    #[test]
    fn articles_filter_case_try_new() {
        let series_id = series::Id::new(1);
        let category_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];
        let tag_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];

        let res = ArticlesFilter::try_new(
            Some(series_id.as_identifier()),
            category_ids.clone(),
            tag_ids.clone(),
        );

        assert!(res.is_ok());

        let filter = res.unwrap();

        assert_eq!(filter.series_identifier(), &Some(series_id.as_identifier()));
        assert_eq!(filter.category_ids(), &category_ids);
        assert_eq!(filter.tag_ids(), &tag_ids);

        let res = ArticlesFilter::try_new(None, vec![], vec![]);

        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, "At least one filter must be specified");
    }
}
