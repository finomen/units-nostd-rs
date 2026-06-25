pub mod errors;
pub mod si;

use core::convert::Infallible;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::marker::{ConstParamTy, PhantomData};
use core::ops::{Add, Div, Mul, Sub};
use num::{NumCast, ToPrimitive};
use paste::paste;

use crate::scale::{ONE, Scale};
use errors::ConversionError;

use si::SiCompoundUnit;
use si::SiCompoundUnitWrapper;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Declare multiplication operation for units
pub trait UnitMul<T> {
    /// Result unit
    type Result;
}

/// Declare division operation for units
pub trait UnitDiv<T> {
    /// Result unit
    type Result;
}

/// Declare addition operation for units
pub trait UnitAdd<T> {
    /// Result unit
    type Result;
}

/// Declare substraction operation for units
pub trait UnitSub<T> {
    /// Result unit
    type Result;
}

/// Declare unit conversion
pub trait UnitConvert<U, T, V, const S1: Scale, const S2: Scale> {
    fn convert(value: T) -> V;
}

/// Declare unit conversion
pub trait UnitTryConvert<U, T, V, const S1: Scale, const S2: Scale> {
    type Error;
    fn try_convert(value: T) -> Result<V, Self::Error>;
}

impl<T, V, const S1: Scale, const S2: Scale, const U: si::SiCompoundUnit>
    UnitConvert<si::SiCompoundUnitWrapper<U>, T, V, S1, S2> for si::SiCompoundUnitWrapper<U>
where
    V: From<T> + From<u64> + Mul<V, Output = V> + Div<V, Output = V>,
{
    fn convert(value: T) -> V {
        let mul = S1 / S2;
        V::from(value) * V::from(mul.numerator()) / V::from(mul.denominator())
    }
}

impl<T, V, const S1: Scale, const S2: Scale, const U: si::SiCompoundUnit>
    UnitTryConvert<si::SiCompoundUnitWrapper<U>, T, V, S1, S2> for si::SiCompoundUnitWrapper<U>
where
    V: NumCast + Mul<V, Output = V> + Div<V, Output = V>,
    T: ToPrimitive,
{
    type Error = ConversionError<ConversionError, ConversionError, ConversionError, Infallible>;
    fn try_convert(value: T) -> Result<V, Self::Error> {
        let mul = S1 / S2;
        let cv = V::from(value).ok_or(ConversionError::ValueConversionError(
            ConversionError::NumCastFailed,
        ))?;
        let cvn = V::from(mul.numerator()).ok_or(ConversionError::NumeratorConversionError(
            ConversionError::NumCastFailed,
        ))?;
        let cvd = V::from(mul.denominator()).ok_or(ConversionError::DenominatorConversionError(
            ConversionError::NumCastFailed,
        ))?;
        Ok(cv * cvn / cvd)
    }
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
    pub const fn new(value: T) -> Self {
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
    pub fn convert<V, const S2: Scale, U2>(self) -> Quantity<V, S2, U2>
    where
        V: Copy,
        U: UnitConvert<U2, T, V, S, S2>,
    {
        Quantity::<V, S2, U2> {
            value: <U as UnitConvert<U2, T, V, S, S2>>::convert(self.value),
            u: PhantomData,
        }
    }

    /// Converts to a different scale `S2` (and value type `V`) like
    /// [`convert`](Self::convert), but uses fallible [`core::convert::TryFrom`]
    /// conversions so it can report failures instead of panicking or
    /// truncating.
    ///
    /// # Errors
    /// Returns [`ConversionError::ValueConversionError`] if the wrapped
    /// value cannot be represented in the target type `V`, or
    /// [`ConversionError::NumeratorConversionError`] /
    /// [`ConversionError::DenominatorConversionError`] if the rescaling
    /// factor's numerator or denominator cannot be represented in `V`.
    ///
    /// # Examples
    /// ```
    /// use units::time::{Minutes, Seconds};
    /// let secs: Result<Seconds<i32>, _> = Minutes::<i32>::new(6).try_convert();
    /// assert_eq!(secs, Ok(Seconds::<i32>::new(360)));
    /// ```
    /// Converting a value that does not fit the target type fails:
    /// ```
    /// use units::time::{Minutes, Seconds};
    /// use units::quantity::errors::{ConversionError};
    ///
    /// let res: Result<Seconds<u8>, _> = Minutes::<u32>::new(300).try_convert();
    /// assert_eq!(
    ///     res,
    ///     Err(ConversionError::ValueConversionError( ConversionError::NumCastFailed)),
    /// );
    /// ```
    pub fn try_convert<V, U2, const S2: Scale, E>(self) -> Result<Quantity<V, S2, U2>, E>
    where
        V: Copy,
        U: UnitTryConvert<U2, T, V, S, S2, Error = E>,
        E: Error,
    {
        Ok(Quantity::<V, S2, U2> {
            value: <U as UnitTryConvert<U2, T, V, S, S2>>::try_convert(self.value())?,
            u: PhantomData,
        })
    }

    /// Current scale relative to the base unit.
    pub const SCALE: Scale = S;

    /// Quantity with same type and unit but different scale.
    pub type WithScale<const S2: Scale> = Quantity<T, S2, U>;
}

impl<T, const S: Scale, const U: SiCompoundUnit> Quantity<T, S, SiCompoundUnitWrapper<U>>
where
    T: Copy,
{
    pub(crate) const UNIT: SiCompoundUnit = U;
}

impl<T, V, const S: Scale, U1, U2> Add<Quantity<V, S, U2>> for Quantity<T, S, U1>
where
    T: Copy,
    V: Copy,
    T: Add<V>,
    U1: UnitAdd<U2>,
{
    type Output = Quantity<<T as Add<V>>::Output, { S }, <U1 as UnitAdd<U2>>::Result>;

    fn add(self, rhs: Quantity<V, S, U2>) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
            u: PhantomData,
        }
    }
}

impl<T, V, const S: Scale, U1, U2> Sub<Quantity<V, S, U2>> for Quantity<T, S, U1>
where
    T: Copy,
    V: Copy,
    T: Sub<V>,
    U1: UnitSub<U2>,
{
    type Output = Quantity<<T as Sub<V>>::Output, { S }, <U1 as UnitSub<U2>>::Result>;

    fn sub(self, rhs: Quantity<V, S, U2>) -> Self::Output {
        Self::Output {
            value: self.value - rhs.value,
            u: PhantomData,
        }
    }
}

impl<T, V, const S1: Scale, const S2: Scale, U1, U2> Mul<Quantity<V, S2, U2>>
    for Quantity<T, S1, U1>
where
    T: Copy,
    V: Copy,
    T: Mul<V>,
    U1: UnitMul<U2>,
    Quantity<<T as Mul<V>>::Output, { S1 * S2 }, <U1 as UnitMul<U2>>::Result>: Sized,
{
    type Output = Quantity<<T as Mul<V>>::Output, { S1 * S2 }, <U1 as UnitMul<U2>>::Result>;

    fn mul(self, rhs: Quantity<V, S2, U2>) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
            u: PhantomData,
        }
    }
}

impl<T, V, const S1: Scale, const S2: Scale, U1, U2> Div<Quantity<V, S2, U2>>
    for Quantity<T, S1, U1>
where
    T: Copy,
    V: Copy,
    T: Div<V>,
    U1: UnitDiv<U2>,
    Quantity<<T as Div<V>>::Output, { S1 / S2 }, <U1 as UnitDiv<U2>>::Result>: Sized,
{
    type Output = Quantity<<T as Div<V>>::Output, { S1 / S2 }, <U1 as UnitDiv<U2>>::Result>;

    fn div(self, rhs: Quantity<V, S2, U2>) -> Self::Output {
        Self::Output {
            value: self.value / rhs.value,
            u: PhantomData,
        }
    }
}

#[cfg(all(test, feature = "time", feature = "length"))]
mod tests {
    use crate::Quantity;
    use crate::base_units::Kelvins;
    use crate::length::*;
    use crate::quantity::errors::ConversionError;
    use crate::quantity::errors::ConversionError::NumCastFailed;
    use crate::quantity::{SiCompoundUnit, SiCompoundUnitWrapper, si};
    use crate::scale::*;
    use crate::temperature::DegreesCelsius;
    use crate::time::*;
    use assert_approx_eq::assert_approx_eq;
    use core::marker::PhantomData;
    use core::num::TryFromIntError;

    #[test]
    fn test_construction() {
        use crate::base_units::Meters;
        assert_eq!(
            Meters::<i32>::new(42),
            Quantity::<i32, ONE, si::Meter<1>> {
                value: 42,
                u: PhantomData
            }
        );
        assert_eq!(
            Meters::<f32>::new(42.5),
            Quantity::<f32, ONE, si::Meter<1>> {
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
                SiCompoundUnitWrapper<{ SiCompoundUnit::zero().meter(1).second(1) }>,
            > {
                value: 18,
                u: PhantomData
            }
        );
        assert_eq!(
            Meters::<i32>::new(6) * Meters::<i32>::new(3),
            Quantity::<i32, ONE, si::Meter<2>> {
                value: 18,
                u: PhantomData,
            }
        );
        assert_eq!(
            KiloMeters::<i32>::new(6) * Minutes::<i32>::new(3),
            Quantity::<
                i32,
                { Scale::new(60000, 1) },
                SiCompoundUnitWrapper<{ SiCompoundUnit::zero().meter(1).second(1) }>,
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
                SiCompoundUnitWrapper<{ SiCompoundUnit::zero().meter(1).second(-1) }>,
            > {
                value: 2,
                u: PhantomData,
            }
        );
        assert_eq!(
            Meters::<i32>::new(6) / Meters::<i32>::new(3),
            Quantity::<i32, ONE, SiCompoundUnitWrapper<{ SiCompoundUnit::zero() }>> {
                value: 2,
                u: PhantomData,
            }
        );
        assert_eq!(
            KiloMeters::<i32>::new(6) / Minutes::<i32>::new(3),
            Quantity::<
                i32,
                { Scale::new(1000, 60) },
                SiCompoundUnitWrapper<{ SiCompoundUnit::zero().meter(1).second(-1) }>,
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
            Err(ConversionError::NumeratorConversionError(NumCastFailed))
        );
        let sec8_res: Result<Seconds<u8>, _> = Seconds::<u32>::new(300).try_convert();
        assert_eq!(
            sec8_res,
            Err(ConversionError::ValueConversionError(NumCastFailed))
        );
        let sec8_res: Result<Quantity<u8, { Scale::new(1000, 1) }, si::Second<1>>, _> =
            Seconds::<u8>::new(1).try_convert();
        assert_eq!(
            sec8_res,
            Err(ConversionError::DenominatorConversionError(NumCastFailed))
        );
    }

    #[cfg(all(test, feature = "temperature"))]
    #[test]
    fn test_celsius_conversion() {
        type DegCelsius = Quantity<i16, { Scale::new(1, 200) }, crate::temperature::Celsius>;
        let celsius: DegCelsius = DegCelsius::new(4800);
        let celsius_f: Result<DegreesCelsius<f32>, _> = celsius.try_convert();
        assert_eq!(celsius_f.unwrap().value(), 24.0);
        let celsius_f: Result<DegreesCelsius<f32>, _> = Kelvins::<i32>::new(293).try_convert();
        assert_approx_eq!(celsius_f.unwrap().value(), 19.85, 1e-3);
    }
}
