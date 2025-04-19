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

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    mod test_page {
        use crate::pagination::Page;

        #[test]
        fn test_page() {
            let page = Page::new(1);
            assert_eq!(page.value(), 1);
        }
        #[test]
        #[should_panic(expected = "Page must be greater than 0")]
        fn test_page_panic() {
            Page::new(0);
        }
        #[test]
        fn test_page_convert() {
            let page = Page::try_from(1);
            assert_eq!(page.is_ok(), true);
            assert_eq!(page.unwrap().value(), 1);

            let page = Page::try_from(0);
            assert_eq!(page.is_err(), true);
            assert_eq!(page.unwrap_err(), "Page must be greater than 0");

            let page = TryInto::<Page>::try_into(1);
            assert_eq!(page.is_ok(), true);
            assert_eq!(page.unwrap().value(), 1);

            let page = TryInto::<Page>::try_into(0);
            assert_eq!(page.is_err(), true);
            assert_eq!(page.unwrap_err(), "Page must be greater than 0");
        }

        #[test]
        fn test_page_display() {
            let page = Page::new(1);
            assert_eq!(page.to_string(), "1");
        }
    }

    mod test_page_size {
        use crate::pagination::PageSize;

        #[test]
        fn test_page_size() {
            let page_size = PageSize::new(1);
            assert_eq!(page_size.value(), 1);
        }
        #[test]
        #[should_panic(expected = "Page size must be greater than 0")]
        fn test_page_size_panic() {
            PageSize::new(0);
        }
        #[test]
        fn test_page_size_convert() {
            let page_size = PageSize::try_from(1);
            assert_eq!(page_size.is_ok(), true);
            assert_eq!(page_size.unwrap().value(), 1);

            let page_size = PageSize::try_from(0);
            assert_eq!(page_size.is_err(), true);
            assert_eq!(page_size.unwrap_err(), "Page size must be greater than 0");

            let page_size = TryInto::<PageSize>::try_into(1);
            assert_eq!(page_size.is_ok(), true);
            assert_eq!(page_size.unwrap().value(), 1);

            let page_size = TryInto::<PageSize>::try_into(0);
            assert_eq!(page_size.is_err(), true);
            assert_eq!(page_size.unwrap_err(), "Page size must be greater than 0");
        }
        #[test]
        fn test_page_size_display() {
            let page_size = PageSize::new(1);
            assert_eq!(page_size.to_string(), "1");
        }
    }

    mod test_offset {
        use crate::pagination::{Offset, Page, PageSize};

        #[test]
        fn test_offset_case_i32() {
            let offset = Offset::new(1);
            assert_eq!(offset.value(), 1);
        }
        #[test]
        #[should_panic(expected = "Offset must be greater than or equal to 0")]
        fn test_offset_case_i32_panic() {
            let _ = Offset::new(-1);
        }
        #[test]
        fn test_offset_case_i32_convert() {
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
        fn test_offset_case_from_page_page_size() {
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
    }
}
