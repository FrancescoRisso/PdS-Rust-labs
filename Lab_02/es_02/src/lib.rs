pub mod solution {
    use core::f32;
    use std::ops::Add;

    pub struct ComplexNumber {
        re: f32,
        im: f32,
    }

    impl ComplexNumber {
        pub fn new(real: f32, imag: f32) -> Self {
            ComplexNumber { re: real, im: imag }
        }

        pub fn from_real(real: f32) -> Self {
            Self::new(real, 0.0)
        }

        pub fn real(&self) -> f32 {
            self.re
        }

        pub fn imag(&self) -> f32 {
            self.im
        }
    }

    impl Add for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.real() + rhs.real(), self.imag() + rhs.imag())
        }
    }
}
