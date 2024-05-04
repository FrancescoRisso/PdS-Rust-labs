use crate::same_amino_acid_sequence::*;

pub struct AmminoacidSequence(Vec<SameAmminoacidSequence>);

impl From<&str> for AmminoacidSequence {
    fn from(value: &str) -> Self {
        let sequence: Vec<SameAmminoacidSequence> =
            value.split(",").map(|amino| amino.into()).collect();
        AmminoacidSequence(sequence)
    }
}

impl AmminoacidSequence {
    pub fn matches<'a>(&self, string: &'a str) -> Option<&'a str> {
        let mut total_len: usize = 0;
        let mut last_ptr: &str = &string;

        for single_amino in &self.0 {
            match single_amino.matches(last_ptr) {
                None => return None,
                Some(ptr) => {
                    total_len += ptr.len();
                    last_ptr = &last_ptr[ptr.len()..];
                }
            }
        }

        return Some(&string[..total_len]);
    }
}
