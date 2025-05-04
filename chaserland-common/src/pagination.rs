use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Page(i32);

impl Page {
    pub fn new(page: i32) -> Self {
        assert!(page > 0, "Page must be greater than 0");
        Page(page)
    }
    pub fn value(&self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for Page {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 1 {
            Err("Page must be greater than 0".to_string())
        } else {
            Ok(Page::new(value))
        }
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct PageSize(i32);

impl PageSize {
    pub fn new(page_size: i32) -> Self {
        assert!(page_size > 0, "Page size must be greater than 0");
        PageSize(page_size)
    }
    pub fn value(&self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for PageSize {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 1 {
            Err("Page size must be greater than 0".to_string())
        } else {
            Ok(PageSize::new(value))
        }
    }
}

impl Display for PageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pagination {
    pub page: Page,
    pub page_size: PageSize,
}

impl Pagination {
    pub fn new(page: Page, page_size: PageSize) -> Self {
        Pagination { page, page_size }
    }
    pub fn as_offset(&self) -> Offset {
        Offset::from((self.page, self.page_size))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Offset(i32);

impl Offset {
    pub fn new(offset: i32) -> Self {
        Self::validate(offset).unwrap();
        Offset(offset)
    }
    pub fn value(&self) -> i32 {
        self.0
    }
    fn validate(offset: i32) -> Result<(), String> {
        match offset >= 0 {
            true => Ok(()),
            false => Err("Offset must be greater than or equal to 0".to_string()),
        }
    }
}

impl TryFrom<i32> for Offset {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match Offset::validate(value) {
            Ok(_) => Ok(Offset(value)),
            Err(err) => Err(err),
        }
    }
}

impl From<(Page, PageSize)> for Offset {
    fn from((page, page_size): (Page, PageSize)) -> Self {
        Offset::new((page.value() - 1) * page_size.value())
    }
}

impl From<Pagination> for Offset {
    fn from(value: Pagination) -> Self {
        Self::from((value.page, value.page_size))
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    mod page {
        use crate::pagination::Page;

        #[test]
        fn page_case_1() {
            let page = Page::new(1);
            assert_eq!(page.value(), 1);
        }
        #[test]
        #[should_panic(expected = "Page must be greater than 0")]
        fn page_case_panic() {
            Page::new(0);
        }
        #[test]
        fn page_case_conversion() {
            let page = Page::try_from(1);
            assert_eq!(page.is_ok(), true);
            assert_eq!(page.unwrap().value(), 1);

            let page = Page::try_from(0);
            assert_eq!(page.is_err(), true);
            assert_eq!(page.unwrap_err(), "Page must be greater than 0");

            let page: Result<Page, String> = 1.try_into();
            assert_eq!(page.is_ok(), true);
            assert_eq!(page.unwrap().value(), 1);

            let page: Result<Page, String> = 0.try_into();
            assert_eq!(page.is_err(), true);
            assert_eq!(page.unwrap_err(), "Page must be greater than 0");
        }

        #[test]
        fn page_case_display() {
            let page = Page::new(1);
            assert_eq!(page.to_string(), "1");
        }
    }

    mod page_size {
        use crate::pagination::PageSize;

        #[test]
        fn page_size_case_1() {
            let page_size = PageSize::new(1);
            assert_eq!(page_size.value(), 1);
        }
        #[test]
        #[should_panic(expected = "Page size must be greater than 0")]
        fn page_size_case_panic() {
            PageSize::new(0);
        }
        #[test]
        fn page_size_case_conversion() {
            let page_size = PageSize::try_from(1);
            assert_eq!(page_size.is_ok(), true);
            assert_eq!(page_size.unwrap().value(), 1);

            let page_size = PageSize::try_from(0);
            assert_eq!(page_size.is_err(), true);
            assert_eq!(page_size.unwrap_err(), "Page size must be greater than 0");

            let page_size: Result<PageSize, String> = 1.try_into();
            assert_eq!(page_size.is_ok(), true);
            assert_eq!(page_size.unwrap().value(), 1);

            let page_size: Result<PageSize, String> = 0.try_into();
            assert_eq!(page_size.is_err(), true);
            assert_eq!(page_size.unwrap_err(), "Page size must be greater than 0");
        }
        #[test]
        fn page_size_case_display() {
            let page_size = PageSize::new(1);
            assert_eq!(page_size.to_string(), "1");
        }
    }

    mod pagination {
        use crate::pagination::Pagination;

        #[test]
        fn pagination_case_1() {
            let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
            assert_eq!(pagination.page.value(), 1);
            assert_eq!(pagination.page_size.value(), 10);
        }

        #[test]
        fn pagination_case_as_offset() {
            let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
            let offset = pagination.as_offset();
            assert_eq!(offset.value(), 0);

            let pagination = Pagination::new(2.try_into().unwrap(), 10.try_into().unwrap());
            let offset = pagination.as_offset();
            assert_eq!(offset.value(), 10);
        }
    }

    mod offset {
        use crate::pagination::{Offset, Page, PageSize, Pagination};

        #[test]
        fn offset_case_1() {
            let offset = Offset::new(1);
            assert_eq!(offset.value(), 1);
        }

        #[test]
        fn offset_case_from_pagination() {
            let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());

            let offset = Offset::from(pagination);
            assert_eq!(offset.value(), 0);

            let pagination = Pagination::new(2.try_into().unwrap(), 10.try_into().unwrap());
            let offset: Offset = pagination.into();
            assert_eq!(offset.value(), 10);
        }

        #[test]
        #[should_panic(expected = "Offset must be greater than or equal to 0")]
        fn offset_case_panic() {
            let _ = Offset::new(-1);
        }

        #[test]
        fn offset_case_conversion() {
            let offset: Result<Offset, _> = 1.try_into();
            assert_eq!(offset.is_ok(), true);
            assert_eq!(offset.unwrap().value(), 1);

            let offset: Result<Offset, _> = (-1).try_into();
            assert_eq!(offset.is_err(), true);
            assert_eq!(
                offset.unwrap_err(),
                "Offset must be greater than or equal to 0"
            );
        }

        #[test]
        fn offset_case_from_page_page_size() {
            let page = Page::new(1);
            let page_size = PageSize::new(10);

            let offset = Offset::from((page, page_size));
            assert_eq!(offset.value(), 0);
            let offset: Offset = (page, page_size).into();
            assert_eq!(offset.value(), 0);

            let page = Page::new(2);
            let page_size = PageSize::new(5);

            let offset = Offset::from((page, page_size));
            assert_eq!(offset.value(), 5);
            let offset: Offset = (page, page_size).into();
            assert_eq!(offset.value(), 5);
        }

        #[test]
        fn offset_case_display() {
            let offset = Offset::new(1);
            assert_eq!(offset.to_string(), "1");
        }
    }
}
