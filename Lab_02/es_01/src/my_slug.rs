use crate::slugify;

pub trait MySlug {
    fn is_slug(&self) -> bool;
}

impl MySlug for String {
    fn is_slug(&self) -> bool {
        slugify(self) == *self
    }
}

impl MySlug for &str {
    fn is_slug(&self) -> bool {
        slugify(self) == *self
    }
}
