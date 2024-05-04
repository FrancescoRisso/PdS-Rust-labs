use std::str::Split;

#[derive(Clone, Copy)]
enum Amminoacids {
    A(),
    C(),
    G(),
    T(),
}

impl Into<char> for Amminoacids {
    fn into(self) -> char {
        match self {
            Amminoacids::A() => 'A',
            Amminoacids::C() => 'C',
            Amminoacids::G() => 'G',
            Amminoacids::T() => 'T',
        }
    }
}

pub struct SameAmminoacidSequence {
    amminoacid: Amminoacids,
    min: usize,
    max: usize,
}

impl From<&str> for SameAmminoacidSequence {
    fn from(value: &str) -> Self {
        let error: String = format!(
            "String {} is not in the format '(A|C|G|T)(min)-(max)'",
            value
        );

        let mut chars = value.chars();

        let letter: char = chars.next().expect(error.as_str());
        let amminoacid: Amminoacids = match letter {
            'A' => Amminoacids::A(),
            'C' => Amminoacids::C(),
            'G' => Amminoacids::G(),
            'T' => Amminoacids::T(),
            _ => panic!("{}", error),
        };

        let numbers: String = chars.collect();
        let mut numbers: Split<&str> = numbers.split("-");

        let min = numbers.next().expect(error.as_str());
        let min = str::parse::<usize>(min).expect(error.as_str());

        let max = numbers.next().expect(error.as_str());
        let max = str::parse::<usize>(max).expect(error.as_str());

        if numbers.next().is_some() {
            panic!("{}", error);
        }

        SameAmminoacidSequence {
            amminoacid,
            min,
            max,
        }
    }
}

impl SameAmminoacidSequence {
    pub fn matches<'a, 'b>(&'a self, string: &'b str) -> Option<&'b str> {
        let mut chars = string.chars();

        for _ in 0..self.min {
            let c = chars.next()?;
            if c != self.amminoacid.into() {
                return None;
            }
        }

        let mut len: usize = self.min;

        for _ in self.min..self.max {
            let c = chars.next();

            if c.is_some_and(|val| val == self.amminoacid.into()) {
                len += 1;
            } else {
                break;
            }
        }

        Some(&string[..len])
    }
}
