pub mod solution {
    use core::f64;
    use std::{
        cmp::Ordering,
        fmt::Debug,
        hash::Hash,
        ops::{Add, AddAssign},
    };

    #[derive(Debug)]
    pub struct ComplexNumber {
        re: f64,
        im: f64,
    }

    impl ComplexNumber {
        pub fn new(real: f64, imag: f64) -> Self {
            ComplexNumber { re: real, im: imag }
        }

        pub fn from_real(real: f64) -> Self {
            Self::new(real, 0.0)
        }

        pub fn real(&self) -> f64 {
            self.re
        }

        pub fn imag(&self) -> f64 {
            self.im
        }

        pub fn to_tuple(&self) -> (f64, f64) {
            (self.real(), self.imag())
        }

        pub fn modulus(&self) -> f64 {
            (self.imag() * self.imag() + self.real() * self.real()).sqrt()
        }
    }

    impl Add for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            self + &rhs
        }
    }

    impl Add<f64> for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: f64) -> Self::Output {
            self + Self::from_real(rhs)
        }
    }

    impl Add<&Self> for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: &Self) -> Self::Output {
            &self + rhs
        }
    }

    impl Add for &ComplexNumber {
        type Output = ComplexNumber;

        fn add(self, rhs: Self) -> Self::Output {
            ComplexNumber::new(self.real() + rhs.real(), self.imag() + rhs.imag())
        }
    }

    impl AddAssign for ComplexNumber {
        fn add_assign(&mut self, rhs: Self) {
            self.re = self.real() + rhs.real();
            self.im = self.imag() + rhs.imag();
        }
    }

    impl Clone for ComplexNumber {
        fn clone(&self) -> Self {
            Self::new(self.real(), self.imag())
        }
    }

    impl Default for ComplexNumber {
        fn default() -> Self {
            Self::from_real(0.0)
        }
    }

    // impl Into<f64> for ComplexNumber {
    //     fn into(self) -> f64 {
    //         if self.imag() == 0.0 {
    //             self.real()
    //         } else {
    //             panic!()
    //         }
    //     }
    // }

    impl TryInto<f64> for ComplexNumber {
        type Error = ();

        fn try_into(self) -> Result<f64, Self::Error> {
            if self.imag() == 0.0 {
                Ok(self.real())
            } else {
                Err(())
            }
        }
    }

    impl From<f64> for ComplexNumber {
        fn from(value: f64) -> Self {
            Self::from_real(value)
        }
    }

    impl Copy for ComplexNumber {}

    impl PartialEq for ComplexNumber {
        fn eq(&self, other: &Self) -> bool {
            self.real() == other.real() && self.imag() == other.imag()
        }
    }

    impl Eq for ComplexNumber {}

    impl PartialOrd for ComplexNumber {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.modulus() - other.modulus() {
                x if x == 0.0 => Some(Ordering::Equal),
                x if x < 0.0 => Some(Ordering::Less),
                x if x > 0.0 => Some(Ordering::Greater),
                _ => None,
            }
        }
    }

    impl Ord for ComplexNumber {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap_or(Ordering::Equal)
        }
    }

    impl AsRef<f64> for ComplexNumber {
        fn as_ref(&self) -> &f64 {
            &self.re
        }
    }

    impl AsMut<f64> for ComplexNumber {
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.re
        }
    }

    impl Hash for ComplexNumber {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            state.write_u64(self.real().to_bits());
            state.write_u64(self.imag().to_bits());
        }
    }
}
