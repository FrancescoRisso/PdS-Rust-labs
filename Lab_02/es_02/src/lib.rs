pub mod solution {
    use core::f64;
    use std::ops::{Add, AddAssign};

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

    impl Into<f64> for ComplexNumber {
        fn into(self) -> f64 {
            self.real()
        }
    }
}
