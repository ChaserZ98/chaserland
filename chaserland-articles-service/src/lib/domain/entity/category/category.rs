use super::{Id, Name, Slug};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Category {
    pub id: Id,
    slug: Slug,
    pub name: Name,
}

impl Category {
    pub fn new(id: Id, name: Name) -> Self {
        let slug = name.as_slug();
        Self { id, slug, name }
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }
}

impl Default for Category {
    fn default() -> Self {
        let id = Id::new(1);
        let name = Name::new("default");
        Self::new(id, name)
    }
}

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
    mod category {
        use super::super::Category;
        #[test]
        fn category_case_new() {
            let category = Category::new(1.try_into().unwrap(), "Category 1".try_into().unwrap());

            assert_eq!(category.id, 1.try_into().unwrap());
            assert_eq!(category.slug, "category-1".try_into().unwrap());
            assert_eq!(category.name, "Category 1".try_into().unwrap());
        }

        #[test]
        fn category_case_default() {
            let category = Category::default();

            assert_eq!(category.id, 1.try_into().unwrap());
            assert_eq!(category.slug, "default".try_into().unwrap());
            assert_eq!(category.name, "default".try_into().unwrap());
        }

        #[test]
        fn category_case_slug() {
            let category = Category::new(1.try_into().unwrap(), "Category 1".try_into().unwrap());
            let slug = "category-1".try_into().unwrap();

            assert_eq!(category.slug(), &slug);
        }
    }
    mod new_category {
        use super::super::NewCategory;

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
}
