use super::{Name, Slug};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewCategory {
    pub name: Name,
    slug: Slug,
}

impl NewCategory {
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
    use super::NewCategory;

    #[test]
    fn new_category_case_new() {
        let new_category = NewCategory::new("Category 1".try_into().unwrap());

        assert_eq!(new_category.name, "Category 1".try_into().unwrap());
        assert_eq!(new_category.slug, "category-1".try_into().unwrap());
    }

    #[test]
    fn new_category_case_slug() {
        let new_category = NewCategory::new("Category 1".try_into().unwrap());
        let slug = "category-1".try_into().unwrap();

        assert_eq!(new_category.slug(), &slug);
    }
}
