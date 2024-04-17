pub mod solution {
    pub struct ComplexNumber {
        re: f32,
        im: f32,
    }

    impl ComplexNumber {
        pub fn new(real: f32, imag: f32) -> Self {
            ComplexNumber { re: real, im: imag }
        }

        pub fn real(&self) -> f32 {
            self.re
        }

        pub fn imag(&self) -> f32 {
            self.im
        }
    }
}
