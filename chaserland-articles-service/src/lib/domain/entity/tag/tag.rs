use super::{Id, Name, Slug};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Tag {
    pub id: Id,
    slug: Slug,
    pub name: Name,
}

impl Tag {
    pub fn new(id: Id, name: Name) -> Self {
        let slug = name.as_slug();
        Self { id, slug, name }
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }
}

impl Default for Tag {
    fn default() -> Self {
        let id = Id::new(1);
        let name = Name::new("default");
        Self::new(id, name)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewTag {
    pub name: Name,
    slug: Slug,
}

impl NewTag {
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
    mod tag {
        use super::super::Tag;

        #[test]
        fn tag_case_new() {
            let tag = Tag::new(1.try_into().unwrap(), "name".try_into().unwrap());
            assert_eq!(tag.id, 1.try_into().unwrap());
            assert_eq!(tag.slug.value(), "name");
            assert_eq!(tag.name.value(), "name");
        }

        #[test]
        fn tag_case_default() {
            let tag = Tag::default();
            assert_eq!(tag.id, 1.try_into().unwrap());
            assert_eq!(tag.slug.value(), "default");
            assert_eq!(tag.name.value(), "default");
        }

        #[test]
        fn tag_case_slug() {
            let tag = Tag::default();
            assert_eq!(tag.slug.value(), "default");
        }
    }

    mod new_tag {
        use super::super::NewTag;

        #[test]
        fn new_tag_case_new() {
            let new_tag = NewTag::new("name".try_into().unwrap());
            assert_eq!(new_tag.name, "name".try_into().unwrap());
            assert_eq!(new_tag.slug, "name".try_into().unwrap());
        }

        #[test]
        fn new_tag_case_slug() {
            let new_tag = NewTag::new("name".try_into().unwrap());
            let slug = "name".try_into().unwrap();
            assert_eq!(new_tag.slug(), &slug);
        }
    }
}
