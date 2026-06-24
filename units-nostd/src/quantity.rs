use crate::scale::{ONE, Scale};

use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::marker::{ConstParamTy, PhantomData};
use core::ops::{Add, Div, Mul, Sub};
use paste::paste;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ConversionError<SE, VE> {
    ScaleFailed(SE),
    ValueConversionFailed(VE),
}

impl<SE, VE> Display for ConversionError<SE, VE>
where
    SE: Display,
    VE: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ConversionError::ScaleFailed(e) => write!(f, "Scale conversion failed: {}", e),
            ConversionError::ValueConversionFailed(e) => {
                write!(f, "Value conversion failed: {}", e)
            }
        }
    }
}
impl<SE, VE> Error for ConversionError<SE, VE>
where
    SE: Display,
    VE: Display,
    SE: Debug,
    VE: Debug,
{
}

macro_rules! si_compound_unit {
    ({ $( $base_unit:ident ),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, ConstParamTy)]
        #[derive_const(PartialEq, Eq)]
        #[doc(hidden)]
        /// Constructed via the provided unit aliases, not directly
        pub struct SiCompoundUnit {
            $(pub(crate)  $base_unit : i32),*
        }

        impl SiCompoundUnit {
            pub(crate) const fn pow(self, p: i32) -> Self {
                Self {
                    $($base_unit: self.$base_unit * p),*
                }
            }
        }

        const impl Mul<SiCompoundUnit> for SiCompoundUnit {
            type Output = SiCompoundUnit;

            fn mul(self, other: SiCompoundUnit) -> Self{
                Self {
                    $($base_unit: self.$base_unit + other.$base_unit),*
                }
            }
        }

        const impl Div<SiCompoundUnit> for SiCompoundUnit {
            type Output = SiCompoundUnit;

            fn div(self, other: SiCompoundUnit) -> Self{
                Self {
                    $($base_unit: self.$base_unit - other.$base_unit),*
                }
            }
        }
    }
}

si_compound_unit!({ meter, second, gram, kelvin, ampere, candela, mole, radians });

#[derive(Debug, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
#[doc(hidden)]
/// Constructed via the provided unit aliases, not directly
pub struct SiCompoundUnitWrapper<const U: SiCompoundUnit> {}

impl<const U: SiCompoundUnit> SiCompoundUnitWrapper<U> {
    pub(crate) const UNIT: SiCompoundUnit = U;
}

#[derive(Debug, Default, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quantity<T, const S: Scale, U> {
    value: T,
    u: PhantomData<U>,
}

impl<T, const S: Scale, U> Clone for Quantity<T, S, U>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            u: PhantomData,
        }
    }
}

impl<T, const S: Scale, U> Copy for Quantity<T, S, U> where T: Copy {}

impl<T, const S: Scale, U> PartialEq<Self> for Quantity<T, S, U>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}
impl<T, const S: Scale, U> Eq for Quantity<T, S, U> where T: Eq {}

impl<T, const S: Scale, U> Hash for Quantity<T, S, U>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<T, const S: Scale, U> Quantity<T, S, U>
where
    T: Copy,
{
    /// Creates a new quantity wrapping `value`.
    ///
    /// # Examples
    /// ```
    /// use units::length::Meters;
    ///
    /// let distance = Meters::new(10);
    /// assert_eq!(distance.value(), 10);
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            value,
            u: PhantomData,
        }
    }

    /// Returns the wrapped value, discarding its unit and scale.
    ///
    /// # Examples
    /// ```
    /// use units::length::Meters;
    ///
    /// assert_eq!(Meters::new(42).value(), 42);
    /// ```
    pub fn value(self) -> T {
        self.value
    }

    /// Converts to a different scale `S2` (and value type `V`), rescaling
    /// the value with infallible [`core::convert::From`] conversions.
    ///
    /// The unit dimensions are preserved; only the scale and the
    /// underlying value type change. Use [`try_convert`](Self::try_convert)
    /// when the target type `V` might not be able to represent the
    /// rescaled value.
    ///
    /// # Examples
    /// ```
    /// use units::time::{Minutes, Seconds};
    ///
    /// // 6 minutes is 360 seconds.
    /// let secs: Seconds<u64> = Minutes::<u64>::new(6).convert();
    /// assert_eq!(secs.value(), 360);
    /// ```
    pub fn convert<V, const S2: Scale>(self) -> Quantity<V, S2, U>
    where
        V: Copy,
        V: From<T>,
        V: From<u64>,
        V: Mul<V, Output = V>,
        V: Div<V, Output = V>,
    {
        let mul = S / S2;
        Quantity::<V, S2, U> {
            value: V::from(self.value) * V::from(mul.numerator()) / V::from(mul.denominator()),
            u: PhantomData,
        }
    }

    /// Converts to a different scale `S2` (and value type `V`) like
    /// [`convert`](Self::convert), but uses fallible [`core::convert::TryFrom`]
    /// conversions so it can report failures instead of panicking or
    /// truncating.
    ///
    /// # Errors
    /// Returns [`ConversionError::ValueConversionFailed`] if the wrapped
    /// value cannot be represented in the target type `V`, or
    /// [`ConversionError::ScaleFailed`] if the rescaling factor's
    /// numerator or denominator cannot be represented in `V`.
    ///
    /// # Examples
    /// ```
    /// use units::time::{Minutes, Seconds};
    /// # fn main() -> Result<(), units::ConversionError<core::num::TryFromIntError, core::convert::Infallible>> {
    /// let secs: Seconds<i32> = Minutes::<i32>::new(6).try_convert()?;
    /// assert_eq!(secs.value(), 360);
    /// # Ok(())
    /// # }
    /// ```
    /// Converting a value that does not fit the target type fails:
    /// ```
    /// use units::time::{Minutes, Seconds};
    /// use units::ConversionError;
    ///
    /// let res: Result<Seconds<u8>, _> = Minutes::<u32>::new(300).try_convert();
    /// assert_eq!(
    ///     res,
    ///     Err(ConversionError::ValueConversionFailed(u8::try_from(300u32).unwrap_err())),
    /// );
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn try_convert<V, const S2: Scale>(
        self,
    ) -> Result<
        Quantity<V, S2, U>,
        ConversionError<<V as TryFrom<u64>>::Error, <V as TryFrom<T>>::Error>,
    >
    where
        V: Copy,
        V: TryFrom<T>,
        V: TryFrom<u64>,
        V: Mul<V, Output = V>,
        V: Div<V, Output = V>,
    {
        let mul = S / S2;
        let cv = V::try_from(self.value).map_err( ConversionError::<<V as TryFrom<u64>>::Error, <V as TryFrom<T>>::Error>::ValueConversionFailed)?;
        let cvn = V::try_from(mul.numerator()).map_err(|e| {
            ConversionError::<<V as TryFrom<u64>>::Error, <V as TryFrom<T>>::Error>::ScaleFailed(e)
        })?;
        let cvd = V::try_from(mul.denominator()).map_err(|e| {
            ConversionError::<<V as TryFrom<u64>>::Error, <V as TryFrom<T>>::Error>::ScaleFailed(e)
        })?;
        Ok(Quantity::<V, S2, U> {
            value: cv * cvn / cvd,
            u: PhantomData,
        })
    }

    pub(crate) const SCALE: Scale = S;
}

impl<T, const S: Scale, const U: SiCompoundUnit> Quantity<T, S, SiCompoundUnitWrapper<U>>
where
    T: Copy,
{
    pub(crate) const UNIT: SiCompoundUnit = U;
}

impl<T, V, const S: Scale, const U: SiCompoundUnit> Add<Quantity<V, S, SiCompoundUnitWrapper<U>>>
    for Quantity<T, S, SiCompoundUnitWrapper<U>>
where
    T: Copy,
    V: Copy,
    T: Add<V>,
{
    type Output = Quantity<<T as Add<V>>::Output, { S }, SiCompoundUnitWrapper<U>>;

    fn add(self, rhs: Quantity<V, S, SiCompoundUnitWrapper<U>>) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
            u: PhantomData,
        }
    }
}

impl<T, V, const S: Scale, const U: SiCompoundUnit> Sub<Quantity<V, S, SiCompoundUnitWrapper<U>>>
    for Quantity<T, S, SiCompoundUnitWrapper<U>>
where
    T: Copy,
    V: Copy,
    T: Sub<V>,
{
    type Output = Quantity<<T as Sub<V>>::Output, { S }, SiCompoundUnitWrapper<U>>;

    fn sub(self, rhs: Quantity<V, S, SiCompoundUnitWrapper<U>>) -> Self::Output {
        Self::Output {
            value: self.value - rhs.value,
            u: PhantomData,
        }
    }
}

impl<T, V, const S1: Scale, const S2: Scale, const U1: SiCompoundUnit, const U2: SiCompoundUnit>
    Mul<Quantity<V, S2, SiCompoundUnitWrapper<U2>>> for Quantity<T, S1, SiCompoundUnitWrapper<U1>>
where
    T: Copy,
    V: Copy,
    T: Mul<V>,
    Quantity<<T as Mul<V>>::Output, { S1 * S2 }, SiCompoundUnitWrapper<{ U1 * U2 }>>: Sized,
{
    type Output = Quantity<<T as Mul<V>>::Output, { S1 * S2 }, SiCompoundUnitWrapper<{ U1 * U2 }>>;

    fn mul(self, rhs: Quantity<V, S2, SiCompoundUnitWrapper<U2>>) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
            u: PhantomData,
        }
    }
}

impl<T, V, const S1: Scale, const S2: Scale, const U1: SiCompoundUnit, const U2: SiCompoundUnit>
    Div<Quantity<V, S2, SiCompoundUnitWrapper<U2>>> for Quantity<T, S1, SiCompoundUnitWrapper<U1>>
where
    T: Copy,
    V: Copy,
    T: Div<V>,
    Quantity<<T as Div<V>>::Output, { S1 / S2 }, SiCompoundUnitWrapper<{ U1 / U2 }>>: Sized,
{
    type Output = Quantity<<T as Div<V>>::Output, { S1 / S2 }, SiCompoundUnitWrapper<{ U1 / U2 }>>;

    fn div(self, rhs: Quantity<V, S2, SiCompoundUnitWrapper<U2>>) -> Self::Output {
        Self::Output {
            value: self.value / rhs.value,
            u: PhantomData,
        }
    }
}

#[cfg(all(test, feature = "time", feature = "length"))]
mod tests {
    use crate::Quantity;
    use crate::length::*;
    use crate::quantity::{ConversionError, SiCompoundUnit, SiCompoundUnitWrapper};
    use crate::scale::*;
    use crate::time::*;
    use core::marker::PhantomData;
    use core::num::TryFromIntError;

    #[test]
    fn test_construction() {
        use crate::base_units::Meters;
        assert_eq!(
            Meters::<i32>::new(42),
            Quantity::<
                i32,
                ONE,
                SiCompoundUnitWrapper<
                    {
                        SiCompoundUnit {
                            meter: 1,
                            second: 0,
                            ampere: 0,
                            candela: 0,
                            gram: 0,
                            kelvin: 0,
                            mole: 0,
                            radians: 0,
                        }
                    },
                >,
            > {
                value: 42,
                u: PhantomData
            }
        );
        assert_eq!(
            Meters::<f32>::new(42.5),
            Quantity::<
                f32,
                ONE,
                SiCompoundUnitWrapper<
                    {
                        SiCompoundUnit {
                            meter: 1,
                            second: 0,
                            ampere: 0,
                            candela: 0,
                            gram: 0,
                            kelvin: 0,
                            mole: 0,
                            radians: 0,
                        }
                    },
                >,
            > {
                value: 42.5,
                u: PhantomData
            }
        );
    }

    #[test]
    fn test_value() {
        use crate::base_units::Meters;
        assert_eq!(Meters::<i32>::new(42).value(), 42);
        assert_eq!(Meters::<f32>::new(42.5).value(), 42.5);
    }

    #[test]
    fn test_mul() {
        use crate::base_units::Meters;
        assert_eq!(
            Meters::<i32>::new(6) * Seconds::<i32>::new(3),
            Quantity::<
                i32,
                ONE,
                SiCompoundUnitWrapper<
                    {
                        SiCompoundUnit {
                            meter: 1,
                            second: 1,
                            ampere: 0,
                            candela: 0,
                            gram: 0,
                            kelvin: 0,
                            mole: 0,
                            radians: 0,
                        }
                    },
                >,
            > {
                value: 18,
                u: PhantomData
            }
        );
        assert_eq!(
            Meters::<i32>::new(6) * Meters::<i32>::new(3),
            Quantity::<
                i32,
                ONE,
                SiCompoundUnitWrapper<
                    {
                        SiCompoundUnit {
                            meter: 2,
                            second: 0,
                            ampere: 0,
                            candela: 0,
                            gram: 0,
                            kelvin: 0,
                            mole: 0,
                            radians: 0,
                        }
                    },
                >,
            > {
                value: 18,
                u: PhantomData,
            }
        );
        assert_eq!(
            KiloMeters::<i32>::new(6) * Minutes::<i32>::new(3),
            Quantity::<
                i32,
                { Scale::new(60000, 1) },
                SiCompoundUnitWrapper<
                    {
                        SiCompoundUnit {
                            meter: 1,
                            second: 1,
                            ampere: 0,
                            candela: 0,
                            gram: 0,
                            kelvin: 0,
                            mole: 0,
                            radians: 0,
                        }
                    },
                >,
            > {
                value: 18,
                u: PhantomData,
            }
        );
    }

    #[test]
    fn test_div() {
        use crate::base_units::Meters;
        assert_eq!(
            Meters::<i32>::new(6) / Seconds::<i32>::new(3),
            Quantity::<
                i32,
                ONE,
                SiCompoundUnitWrapper<
                    {
                        SiCompoundUnit {
                            meter: 1,
                            second: -1,
                            ampere: 0,
                            candela: 0,
                            gram: 0,
                            kelvin: 0,
                            mole: 0,
                            radians: 0,
                        }
                    },
                >,
            > {
                value: 2,
                u: PhantomData,
            }
        );
        assert_eq!(
            Meters::<i32>::new(6) / Meters::<i32>::new(3),
            Quantity::<
                i32,
                ONE,
                SiCompoundUnitWrapper<
                    {
                        SiCompoundUnit {
                            meter: 0,
                            second: 0,
                            ampere: 0,
                            candela: 0,
                            gram: 0,
                            kelvin: 0,
                            mole: 0,
                            radians: 0,
                        }
                    },
                >,
            > {
                value: 2,
                u: PhantomData,
            }
        );
        assert_eq!(
            KiloMeters::<i32>::new(6) / Minutes::<i32>::new(3),
            Quantity::<
                i32,
                { Scale::new(1000, 60) },
                SiCompoundUnitWrapper<
                    {
                        SiCompoundUnit {
                            meter: 1,
                            second: -1,
                            ampere: 0,
                            candela: 0,
                            gram: 0,
                            kelvin: 0,
                            mole: 0,
                            radians: 0,
                        }
                    },
                >,
            > {
                value: 2,
                u: PhantomData,
            }
        );
    }

    #[test]
    fn test_add() {
        use crate::base_units::Meters;
        assert_eq!(
            Meters::<i32>::new(6) + Meters::<i32>::new(3),
            Quantity::<
                i32,
                ONE,
                SiCompoundUnitWrapper<
                    {
                        SiCompoundUnit {
                            meter: 1,
                            second: 0,
                            ampere: 0,
                            candela: 0,
                            gram: 0,
                            kelvin: 0,
                            mole: 0,
                            radians: 0,
                        }
                    },
                >,
            > {
                value: 9,
                u: PhantomData,
            }
        );
    }

    #[test]
    fn test_sub() {
        use crate::base_units::Meters;
        assert_eq!(
            Meters::<i32>::new(6) - Meters::<i32>::new(3),
            Quantity::<
                i32,
                ONE,
                SiCompoundUnitWrapper<
                    {
                        SiCompoundUnit {
                            meter: 1,
                            second: 0,
                            ampere: 0,
                            candela: 0,
                            gram: 0,
                            kelvin: 0,
                            mole: 0,
                            radians: 0,
                        }
                    },
                >,
            > {
                value: 3,
                u: PhantomData,
            }
        );
    }

    #[test]
    fn test_convert() {
        use crate::base_units::Meters;
        let sec: Seconds<u64> = Minutes::<u64>::new(6).convert();
        assert_eq!(sec.value(), 360);
    }

    #[test]
    fn test_try_convert() {
        use crate::base_units::Meters;
        let sec: Seconds<i32> = Minutes::<i32>::new(6).try_convert().unwrap();
        assert_eq!(sec.value(), 360);
        let sec8_res: Result<Seconds<u8>, _> = Hours::<u8>::new(1).try_convert();
        assert_eq!(
            sec8_res,
            Err(ConversionError::ScaleFailed(
                u8::try_from(300u32).unwrap_err()
            ))
        );
        let sec8_res: Result<Seconds<u8>, _> = Seconds::<u32>::new(300).try_convert();
        assert_eq!(
            sec8_res,
            Err(ConversionError::ValueConversionFailed(
                u8::try_from(300u32).unwrap_err()
            ))
        );
    }
}
