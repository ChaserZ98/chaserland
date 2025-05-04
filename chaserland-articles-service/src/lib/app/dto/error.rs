use crate::domain::entity::{category, series, tag};

#[derive(Debug, thiserror::Error)]
pub enum DTOError {
    #[error(transparent)]
    Article(#[from] ArticleDTOError),
}

impl DTOError {
    pub fn is_article_error(&self) -> bool {
        matches!(self, DTOError::Article(_))
    }

    pub fn as_article_error(&self) -> Option<&ArticleDTOError> {
        let Self::Article(e) = self;
        Some(e)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ArticleDTOError {
    #[error("Article is not defined")]
    ArticleNotDefined,
    #[error(
        "Series mismatch: article.series_id is {0} but series_id is {1}", article_series_id.map(|x| x.to_string()).unwrap_or("None".to_string()),
        series_id.map(|x| x.to_string()).unwrap_or("None".to_string())
    )]
    SeriesMismatch {
        article_series_id: Option<series::Id>,
        series_id: Option<series::Id>,
    },
    #[error(
        "Categories length mismatch: article.category_ids has length {0} but categories length is {1}",
        article_categories_length,
        categories_length
    )]
    CategoriesLengthMismatch {
        article_categories_length: usize,
        categories_length: usize,
    },
    #[error(
        "Categories element mismatch: article.category_ids has category with id {} but categories has no such category",
        0
    )]
    CategoriesElementMismatch(category::Id),
    #[error(
        "Tags length mismatch: article.tag_ids has length {} but tags length is {}",
        article_tags_length,
        tags_length
    )]
    TagsLengthMismatch {
        article_tags_length: usize,
        tags_length: usize,
    },
    #[error(
        "Tags element mismatch: article.tag_ids has tag with id {} but tags has no such tag",
        0
    )]
    TagsElementMismatch(tag::Id),
}

impl ArticleDTOError {
    pub fn is_article_not_defined(&self) -> bool {
        matches!(self, ArticleDTOError::ArticleNotDefined)
    }

    pub fn is_series_mismatch(&self) -> bool {
        matches!(self, ArticleDTOError::SeriesMismatch { .. })
    }

    pub fn is_categories_length_mismatch(&self) -> bool {
        matches!(self, ArticleDTOError::CategoriesLengthMismatch { .. })
    }

    pub fn is_categories_element_mismatch(&self) -> bool {
        matches!(self, ArticleDTOError::CategoriesElementMismatch(_))
    }

    pub fn is_tags_length_mismatch(&self) -> bool {
        matches!(self, ArticleDTOError::TagsLengthMismatch { .. })
    }

    pub fn is_tags_element_mismatch(&self) -> bool {
        matches!(self, ArticleDTOError::TagsElementMismatch(_))
    }

    pub fn as_article_not_defined(&self) -> Option<&ArticleDTOError> {
        match self {
            Self::ArticleNotDefined => Some(self),
            _ => None,
        }
    }

    pub fn as_series_mismatch(&self) -> Option<&ArticleDTOError> {
        match self {
            Self::SeriesMismatch { .. } => Some(self),
            _ => None,
        }
    }

    pub fn as_categories_length_mismatch(&self) -> Option<&ArticleDTOError> {
        match self {
            Self::CategoriesLengthMismatch { .. } => Some(self),
            _ => None,
        }
    }

    pub fn as_categories_element_mismatch(&self) -> Option<&ArticleDTOError> {
        match self {
            Self::CategoriesElementMismatch(_) => Some(self),
            _ => None,
        }
    }

    pub fn as_tags_length_mismatch(&self) -> Option<&ArticleDTOError> {
        match self {
            Self::TagsLengthMismatch { .. } => Some(self),
            _ => None,
        }
    }

    pub fn as_tags_element_mismatch(&self) -> Option<&ArticleDTOError> {
        match self {
            Self::TagsElementMismatch(_) => Some(self),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ArticleDTOError, DTOError};

    #[test]
    fn dto_error_case_article_error() {
        let err = DTOError::Article(ArticleDTOError::ArticleNotDefined);

        assert!(err.is_article_error());

        let err = err.as_article_error();

        assert!(err.is_some());

        let err = err.unwrap();

        assert!(matches!(err, ArticleDTOError::ArticleNotDefined));
    }

    #[test]
    fn article_dto_error_case_article_not_defined() {
        let err = ArticleDTOError::ArticleNotDefined;

        assert!(err.is_article_not_defined());

        let err = err.as_article_not_defined();

        assert!(err.is_some());

        let err = err.unwrap();

        assert!(matches!(err, ArticleDTOError::ArticleNotDefined));

        let err = ArticleDTOError::SeriesMismatch {
            article_series_id: Some(1.try_into().unwrap()),
            series_id: Some(2.try_into().unwrap()),
        };

        assert_eq!(err.is_article_not_defined(), false);
    }

    #[test]
    fn article_dto_error_case_series_mismatch() {
        let err = ArticleDTOError::SeriesMismatch {
            article_series_id: Some(1.try_into().unwrap()),
            series_id: Some(2.try_into().unwrap()),
        };

        assert!(err.is_series_mismatch());

        let err = err.as_series_mismatch();

        assert!(err.is_some());

        let err = err.unwrap();

        assert!(matches!(err, ArticleDTOError::SeriesMismatch { .. }));

        let err = ArticleDTOError::CategoriesLengthMismatch {
            article_categories_length: 1,
            categories_length: 2,
        };

        assert_eq!(err.is_series_mismatch(), false);

        let err = err.as_series_mismatch();

        assert!(err.is_none());
    }

    #[test]
    fn article_dto_error_case_categories_length_mismatch() {
        let err = ArticleDTOError::CategoriesLengthMismatch {
            article_categories_length: 1,
            categories_length: 2,
        };

        assert!(err.is_categories_length_mismatch());

        let err = err.as_categories_length_mismatch();

        assert!(err.is_some());

        let err = err.unwrap();

        assert!(matches!(
            err,
            ArticleDTOError::CategoriesLengthMismatch { .. }
        ));

        let err = ArticleDTOError::CategoriesElementMismatch(1.try_into().unwrap());

        assert_eq!(err.is_categories_length_mismatch(), false);

        let err = err.as_categories_length_mismatch();

        assert!(err.is_none());
    }

    #[test]
    fn article_dto_error_case_categories_element_mismatch() {
        let err = ArticleDTOError::CategoriesElementMismatch(1.try_into().unwrap());

        assert!(err.is_categories_element_mismatch());

        let err = err.as_categories_element_mismatch();

        assert!(err.is_some());

        let err = err.unwrap();

        assert!(matches!(err, ArticleDTOError::CategoriesElementMismatch(_)));

        let err = ArticleDTOError::TagsLengthMismatch {
            article_tags_length: 1,
            tags_length: 2,
        };

        assert_eq!(err.is_categories_element_mismatch(), false);

        let err = err.as_categories_element_mismatch();

        assert!(err.is_none());
    }

    #[test]
    fn article_dto_error_case_tags_length_mismatch() {
        let err = ArticleDTOError::TagsLengthMismatch {
            article_tags_length: 1,
            tags_length: 2,
        };

        assert!(err.is_tags_length_mismatch());

        let err = err.as_tags_length_mismatch();

        assert!(err.is_some());

        let err = err.unwrap();

        assert!(matches!(err, ArticleDTOError::TagsLengthMismatch { .. }));

        let err = ArticleDTOError::TagsElementMismatch(1.try_into().unwrap());

        assert_eq!(err.is_tags_length_mismatch(), false);

        let err = err.as_tags_length_mismatch();

        assert!(err.is_none());
    }

    #[test]
    fn article_dto_error_case_tags_element_mismatch() {
        let err = ArticleDTOError::TagsElementMismatch(1.try_into().unwrap());

        assert!(err.is_tags_element_mismatch());

        let err = err.as_tags_element_mismatch();

        assert!(err.is_some());

        let err = err.unwrap();

        assert!(matches!(err, ArticleDTOError::TagsElementMismatch(_)));

        let err = ArticleDTOError::ArticleNotDefined;

        assert_eq!(err.is_tags_element_mismatch(), false);

        let err = err.as_tags_element_mismatch();

        assert!(err.is_none());
    }
}
