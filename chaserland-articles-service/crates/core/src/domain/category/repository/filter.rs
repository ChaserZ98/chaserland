use crate::domain::category::vo as category;

#[derive(Debug)]
pub struct CategoriesFilter {
    category_ids: Vec<category::Id>,
}

impl CategoriesFilter {
    pub fn new(category_ids: Vec<category::Id>) -> Self {
        Self::validate(&category_ids).unwrap();
        Self { category_ids }
    }

    pub fn try_new(category_ids: Vec<category::Id>) -> Result<Self, String> {
        Self::validate(&category_ids)?;

        Ok(Self { category_ids })
    }

    pub fn category_ids(&self) -> &Vec<category::Id> {
        &self.category_ids
    }

    fn validate(category_ids: &Vec<category::Id>) -> Result<(), String> {
        match category_ids.is_empty() {
            true => Err("At least one category id must be specified".to_string()),
            false => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CategoriesFilter;

    #[test]
    fn categories_filter_case_new() {
        let category_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];

        let filter = CategoriesFilter::new(category_ids.clone());

        assert_eq!(filter.category_ids(), &category_ids);
    }

    #[test]
    #[should_panic(expected = "At least one category id must be specified")]
    fn categories_filter_case_new_panic() {
        CategoriesFilter::new(vec![]);
    }
}
