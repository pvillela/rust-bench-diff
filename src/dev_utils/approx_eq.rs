//! Provides approximate equality for floating point types

pub trait ApproxEq {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool;
    fn round_to_sig_decimals(self, n: u32) -> Self;
}

impl ApproxEq for f32 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        (self - other).abs() <= epsilon
    }

    fn round_to_sig_decimals(self, n: u32) -> f32 {
        let pow = 10.0_f32.powi(n as i32);
        (self * pow).round() / pow
    }
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        (self - other).abs() < epsilon
    }

    fn round_to_sig_decimals(self, n: u32) -> f64 {
        let pow = 10.0_f64.powi(n as i32);
        (self * pow).round() / pow
    }
}

#[cfg(test)]
mod test {
    use super::ApproxEq;

    #[test]
    fn test() {
        {
            let x32: f32 = 123.444444;
            let y32: f32 = 123.444454;
            let z32: f32 = 123.444455;
            let epsilon: f32 = 0.00001;

            assert!(x32.approx_eq(y32, epsilon), "x32 must be approx_eq to y32");
            assert!(
                !x32.approx_eq(z32, epsilon),
                "x32 must not be approx_eq to z32"
            );
        }

        {
            let x64: f64 = 123.444444;
            let y64: f64 = 123.444454;
            let z64: f64 = 123.444455;
            let epsilon: f64 = 0.00001;

            assert!(x64.approx_eq(y64, epsilon), "x64 must be approx_eq to y64");
            assert!(
                !x64.approx_eq(z64, epsilon),
                "x64 must not be approx_eq to z64"
            );
        }
    }
}
