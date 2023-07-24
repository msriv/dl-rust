pub trait ToF64 {
    fn to_f64(self) -> f64;
}

impl ToF64 for i8 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}
