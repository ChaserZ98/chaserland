use crate::domain::tag::vo as tag;

#[derive(Debug)]
pub struct TagsFilter {
    tag_ids: Vec<tag::Id>,
}

impl TagsFilter {
    pub fn new(tag_ids: Vec<tag::Id>) -> Self {
        Self::validate(&tag_ids).unwrap();
        Self { tag_ids }
    }

    pub fn tag_ids(&self) -> &Vec<tag::Id> {
        &self.tag_ids
    }

    fn validate(tag_ids: &Vec<tag::Id>) -> Result<(), String> {
        match tag_ids.is_empty() {
            true => Err("At least one tag id must be specified".to_string()),
            false => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TagsFilter;

    #[test]
    fn tags_filter_case_new() {
        let tag_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];
        let tag_filter = TagsFilter::new(tag_ids.clone());

        assert_eq!(tag_filter.tag_ids(), &tag_ids);
    }

    #[test]
    #[should_panic(expected = "At least one tag id must be specified")]
    fn tags_filter_case_new_panic() {
        TagsFilter::new(vec![]);
    }
}
