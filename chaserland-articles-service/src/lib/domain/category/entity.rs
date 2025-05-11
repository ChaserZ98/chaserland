use super::vo::{Id, Name, Slug};

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

#[cfg(test)]
mod tests {
    use super::Category;
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
