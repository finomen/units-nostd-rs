use core::fmt::{Display, Formatter};
use core::marker::ConstParamTy;
use core::ops::{Div, Mul};

#[derive(Debug, Clone, Copy, ConstParamTy)]
#[derive_const(PartialEq, Eq)]
#[doc(hidden)]
/// Scale relative to base unit
pub struct Rational {
    numerator: u64,
    denominator: u64,
}

impl Rational {
    /// Create new scale. Scale will be normalized
    /// ```
    /// assert_eq!(units::Rational::new(2, 4), units::Rational::new(1, 2));
    /// ```
    pub const fn new(numerator: u64, denominator: u64) -> Rational {
        Rational {
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

    const fn normalized(&self) -> Rational {
        let gcd = gcd::binary_u64(self.numerator, self.denominator);
        Rational {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }

    pub(crate) const fn pow(self, p: i32) -> Rational {
        if p >= 0 {
            Rational {
                numerator: self.numerator.pow(p as u32),
                denominator: self.denominator.pow(p as u32),
            }
            .normalized()
        } else {
            Rational {
                numerator: self.denominator.pow(-p as u32),
                denominator: self.numerator.pow(-p as u32),
            }
            .normalized()
        }
    }

    pub(crate) const fn nth_root(self, p: i32) -> Rational {
        const fn integer_nth_root(val: u64, n: u32) -> u64 {
            // Base cases
            if val <= 1 {
                return val;
            }
            if n == 0 {
                panic!("Cannot calculate 0th root");
            }
            if n == 1 {
                return val;
            }
            // Since 2^64 > u64::MAX, any root >= 64 of a u64 > 1 will always be 1
            if n >= 64 {
                return 1;
            }

            let mut low = 1;
            let mut high = val;
            let mut ans = 1;

            while low <= high {
                let mid = low + (high - low) / 2;

                // Calculate mid^n safely, checking for overflow at each multiplication step
                let mut temp = 1u64;
                let mut overflowed = false;
                let mut i = 0;

                while i < n {
                    let (res, over) = temp.overflowing_mul(mid);
                    if over {
                        overflowed = true;
                        break;
                    }
                    temp = res;
                    i += 1;
                }

                if overflowed || temp > val {
                    // Mid is too high, or mid^n overflowed u64 bounds
                    high = mid - 1;
                } else if temp < val {
                    // Mid might be the answer, but try going higher
                    ans = mid;
                    low = mid + 1;
                } else {
                    // Exact match found
                    return mid;
                }
            }

            ans
        }
        if p > 0 {
            Self {
                numerator: integer_nth_root(self.numerator, p as u32),
                denominator: integer_nth_root(self.denominator, p as u32),
            }
        } else {
            Self {
                numerator: integer_nth_root(self.denominator, p.unsigned_abs()),
                denominator: integer_nth_root(self.numerator, p.unsigned_abs()),
            }
        }
    }
}

const impl Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
        .normalized()
    }
}

const impl Div for Rational {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        }
        .normalized()
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if self.denominator == 1 {
            self.numerator.fmt(f)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn scale_new() {
        assert_eq!(
            Rational::new(1, 2),
            Rational {
                numerator: 1,
                denominator: 2
            }
        );
        assert_eq!(
            Rational::new(8, 6),
            Rational {
                numerator: 4,
                denominator: 3
            }
        );
    }

    #[test]
    fn scale_pow() {
        assert_eq!(
            Rational::new(2, 3).pow(3),
            Rational {
                numerator: 8,
                denominator: 27
            }
        );
        assert_eq!(
            Rational::new(5, 6).pow(-2),
            Rational {
                numerator: 36,
                denominator: 25
            }
        );
    }

    #[test]
    fn scale_mul() {
        assert_eq!(
            Rational::new(2, 3) * Rational::new(7, 6),
            Rational {
                numerator: 7,
                denominator: 9
            }
        );
    }

    #[test]
    fn scale_div() {
        assert_eq!(
            Rational::new(2, 3) / Rational::new(7, 6),
            Rational {
                numerator: 4,
                denominator: 7
            }
        );
    }

    #[test]
    fn scale_fmt() {
        assert_eq!(format!("{}", Rational::new(6, 3)), "2");
        assert_eq!(format!("{}", Rational::new(1, 2)), "1/2");
    }

    #[test]
    fn scale_nth_root() {
        assert_eq!(Rational::new(27, 8).nth_root(1), Rational::new(27, 8));
        assert_eq!(Rational::new(27, 8).nth_root(3), Rational::new(3, 2));
        assert_eq!(Rational::new(27, 8).nth_root(-3), Rational::new(2, 3));
        assert_eq!(Rational::new(27, 8).nth_root(128), Rational::new(1, 1));
        assert_eq!(Rational::new(1, 1).nth_root(2), Rational::new(1, 1));
    }
    #[test]
    #[should_panic(expected = "Cannot calculate 0th root")]
    fn scale_zero_nth_root() {
        Rational::new(2, 1).nth_root(0);
    }

    #[test]
    fn scale_accessors() {
        let r = Rational::new(8, 6);
        assert_eq!(r.numerator(), 4);
        assert_eq!(r.denominator(), 3);
    }

    #[test]
    fn scale_nth_root_inexact_with_overflow() {
        assert_eq!(
            Rational::new(u64::MAX, 1).nth_root(2),
            Rational::new(4294967295, 1)
        );
    }
}
