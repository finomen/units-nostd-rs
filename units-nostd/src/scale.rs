use core::marker::ConstParamTy;
use core::ops::{Div, Mul};

#[derive(Debug, Clone, Copy, ConstParamTy)]
#[derive_const(PartialEq, Eq)]
#[doc(hidden)]
/// Constructed via the provided unit aliases, not directly
pub struct Scale {
    numerator: u64,
    denominator: u64,
}

impl Scale {
    pub(crate) const fn new(numerator: u64, denominator: u64) -> Scale {
        Scale {
            numerator,
            denominator,
        }
        .normalized()
    }

    pub(crate) const fn numerator(&self) -> u64 {
        self.numerator
    }

    pub(crate) const fn denominator(&self) -> u64 {
        self.denominator
    }

    const fn normalized(&self) -> Scale {
        let gcd = gcd::binary_u64(self.numerator, self.denominator);
        Scale {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }

    pub(crate) const fn pow(self, p: i32) -> Scale {
        if p >= 0 {
            Scale {
                numerator: self.numerator.pow(p as u32),
                denominator: self.denominator.pow(p as u32),
            }
            .normalized()
        } else {
            Scale {
                numerator: self.denominator.pow(-p as u32),
                denominator: self.numerator.pow(-p as u32),
            }
            .normalized()
        }
    }
}

pub(crate) const ONE: Scale = Scale::new(1, 1);

const impl Mul for Scale {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
        .normalized()
    }
}

const impl Div for Scale {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        }
        .normalized()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_new() {
        assert_eq!(
            Scale::new(1, 2),
            Scale {
                numerator: 1,
                denominator: 2
            }
        );
        assert_eq!(
            Scale::new(8, 6),
            Scale {
                numerator: 4,
                denominator: 3
            }
        );
    }

    #[test]
    fn scale_pow() {
        assert_eq!(
            Scale::new(2, 3).pow(3),
            Scale {
                numerator: 8,
                denominator: 27
            }
        );
        assert_eq!(
            Scale::new(5, 6).pow(-2),
            Scale {
                numerator: 36,
                denominator: 25
            }
        );
    }

    #[test]
    fn scale_mul() {
        assert_eq!(
            Scale::new(2, 3) * Scale::new(7, 6),
            Scale {
                numerator: 7,
                denominator: 9
            }
        );
    }

    #[test]
    fn scale_div() {
        assert_eq!(
            Scale::new(2, 3) / Scale::new(7, 6),
            Scale {
                numerator: 4,
                denominator: 7
            }
        );
    }
}
