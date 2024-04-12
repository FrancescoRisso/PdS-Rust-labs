use crate::slugify;

pub trait MySlug {
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

// impl MySlug for String {
//     fn is_slug(&self) -> bool {
//         slugify(self) == *self
//     }

//     fn to_slug(&self) -> String {
//         slugify(self)
//     }
// }

// impl MySlug for &str {
//     fn is_slug(&self) -> bool {
//         slugify(self) == *self
//     }

//     fn to_slug(&self) -> String {
//         slugify(self)
//     }
// }

impl<T> MySlug for T
where
    T: AsRef<str>,
{
    fn is_slug(&self) -> bool {
        slugify(self.as_ref()) == *self.as_ref()
    }

    fn to_slug(&self) -> String {
        slugify(self.as_ref())
    }
}
