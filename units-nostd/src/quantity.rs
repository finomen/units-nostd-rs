pub mod errors;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Sub};
use num::{NumCast, ToPrimitive};

use crate::scale::Rational;
use errors::ConversionError;

use crate::composite_unit::{UnitDiv, UnitMul};
use crate::quantity::errors::ConversionError::{
    DenominatorConversionError, NumeratorConversionError, ValueConversionError,
};
use crate::unit::helpers::Scale;
use crate::unit::{Tag, Unit, UnitTagMarker, UnitTags};

pub trait UnitConvert<T, V> {
    fn convert(v: T) -> V;
}

pub trait UnitTryConvert<T, V> {
    type Error;
    fn try_convert(v: T) -> Result<V, Self::Error>;
}

impl<T, V, U, S1, S2> UnitConvert<T, V> for (U, S1, U, S2)
where
    U: UnitTagMarker,
    S1: Tag<Rational>,
    S2: Tag<Rational>,
    T: ToPrimitive,
    V: NumCast + Mul<Output = V> + Div<Output = V>,
    S1: Tag<Rational>,
    S2: Tag<Rational>,
{
    fn convert(v: T) -> V {
        let rescale = S1::VALUE / S2::VALUE;
        V::from(v).unwrap() * V::from(rescale.numerator()).unwrap()
            / V::from(rescale.denominator()).unwrap()
    }
}

impl<T, V, U, S1, S2> UnitTryConvert<T, V> for (U, S1, U, S2)
where
    U: UnitTagMarker,
    S1: Tag<Rational>,
    S2: Tag<Rational>,
    V: NumCast + Mul<Output = V> + Div<Output = V>,
    S1: Tag<Rational>,
    S2: Tag<Rational>,
    T: ToPrimitive,
{
    type Error = ConversionError<ConversionError, ConversionError, ConversionError>;
    fn try_convert(v: T) -> Result<V, Self::Error> {
        let rescale = S1::VALUE / S2::VALUE;

        let v_c: V = V::from(v).ok_or(ValueConversionError(ConversionError::NumCastFailed))?;
        let n_c: V = V::from(rescale.numerator())
            .ok_or(NumeratorConversionError(ConversionError::NumCastFailed))?;
        let d_c: V = V::from(rescale.denominator())
            .ok_or(DenominatorConversionError(ConversionError::NumCastFailed))?;
        Ok(v_c * n_c / d_c)
    }
}

/// Quantity/unit information
pub trait QuantityInfo {
    /// Unit
    type Unit: Unit;
    /// Rescale unit
    type Scaled<const S: Rational>;
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quantity<T, US> {
    value: T,
    _p: PhantomData<US>,
}

// The unit type `US` is a zero-sized phantom marker, so every one of these
// traits depends only on `T`. They are written by hand rather than derived so
// the bounds do not spuriously require `US: Trait` — the marker unit types do
// not all implement them (e.g. composite products are neither `Eq` nor `Ord`),
// which would otherwise strip these traits from real quantities.
impl<T, US> Debug for Quantity<T, US>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Quantity")
            .field("value", &self.value)
            .finish()
    }
}

impl<T, US> Clone for Quantity<T, US>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _p: PhantomData,
        }
    }
}

impl<T, US> Copy for Quantity<T, US> where T: Copy {}

impl<T, US> Default for Quantity<T, US>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            value: T::default(),
            _p: PhantomData,
        }
    }
}

impl<T, US> PartialEq for Quantity<T, US>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T, US> Eq for Quantity<T, US> where T: Eq {}

impl<T, US> PartialOrd for Quantity<T, US>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T, US> Ord for Quantity<T, US>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T, US> Hash for Quantity<T, US>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T, US, U, S> Quantity<T, US>
where
    T: Copy,
    US: Unit + UnitTags<Unit = U, SymbolScale = S>,
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
            _p: PhantomData,
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

    /// Relabels the quantity as a different unit type `U2` that shares the same
    /// normalized dimension and symbol scale.
    ///
    /// Only the label (and therefore the [`Display`] output) changes; the value
    /// is copied through untouched. This is useful to present the result of an
    /// arithmetic expression as a named unit — for example viewing `m / m` as a
    /// plain [`Unitless`](crate::Unitless) ratio.
    ///
    /// # Examples
    /// ```
    /// use units::length::Meters;
    ///
    /// let ratio: units::Unitless<i32> = (Meters::new(6) / Meters::new(3)).alias();
    /// assert_eq!(ratio.value(), 2);
    /// ```
    pub fn alias<U2>(&self) -> Quantity<T, U2>
    where
        U2: Unit + UnitTags<Unit = US::Unit, SymbolScale = US::SymbolScale>,
    {
        Quantity {
            value: self.value,
            _p: PhantomData,
        }
    }

    /// Converts to a different scale `S2` (and value type `V`), rescaling
    /// the value with `num::NumCast`.
    ///
    /// The unit dimensions are preserved; only the scale and the
    /// underlying value type change. The cast is unwrapped, so this
    /// **panics** if the rescaled value cannot be represented in `V`; use
    /// [`try_convert`](Self::try_convert) when that can happen.
    ///
    /// # Examples
    /// ```
    /// use units::time::{Minutes, Seconds};
    ///
    /// // 6 minutes is 360 seconds.
    /// let secs: Seconds<u64> = Minutes::<u64>::new(6).convert();
    /// assert_eq!(secs.value(), 360);
    /// ```
    pub fn convert<V, US2, U2, S2>(self) -> Quantity<V, US2>
    where
        V: Copy,
        US2: Unit + UnitTags<Unit = U2, SymbolScale = S2>,
        (U, S, U2, S2): UnitConvert<T, V>,
    {
        Quantity::<V, US2> {
            value: <(U, S, U2, S2) as UnitConvert<T, V>>::convert(self.value),
            _p: PhantomData,
        }
    }
    /// Converts to a different scale `S2` (and value type `V`) like
    /// [`convert`](Self::convert), but performs the `num::NumCast`
    /// conversions fallibly, returning an error instead of panicking when a
    /// value cannot be represented in `V`.
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
    pub fn try_convert<V, US2, U2, S2, E>(self) -> Result<Quantity<V, US2>, E>
    where
        V: Copy,
        US2: Unit + UnitTags<Unit = U2, SymbolScale = S2>,
        (U, S, U2, S2): UnitTryConvert<T, V, Error = E>,
        E: Error,
    {
        Ok(Quantity::<V, US2> {
            value: <(U, S, U2, S2) as UnitTryConvert<T, V>>::try_convert(self.value())?,
            _p: PhantomData,
        })
    }
}

impl<T, US: Unit> QuantityInfo for Quantity<T, US> {
    type Unit = US;
    type Scaled<const S: Rational> = Quantity<T, Scale<US, S>>;
}

impl<T, V, U> Add<Quantity<V, U>> for Quantity<T, U>
where
    T: Copy,
    V: Copy,
    T: Add<V>,
{
    type Output = Quantity<<T as Add<V>>::Output, U>;

    fn add(self, rhs: Quantity<V, U>) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
            _p: PhantomData,
        }
    }
}

impl<T, V, U> Sub<Quantity<V, U>> for Quantity<T, U>
where
    T: Copy,
    V: Copy,
    T: Sub<V>,
    U: Unit,
{
    type Output = Quantity<<T as Sub<V>>::Output, U>;

    fn sub(self, rhs: Quantity<V, U>) -> Self::Output {
        Self::Output {
            value: self.value - rhs.value,
            _p: PhantomData,
        }
    }
}

impl<T, V, U1, U2> Mul<Quantity<V, U2>> for Quantity<T, U1>
where
    T: Copy,
    V: Copy,
    T: Mul<V>,
    U1: Unit,
    U2: Unit,
{
    type Output = Quantity<<T as Mul<V>>::Output, UnitMul<U1, U2>>;

    fn mul(self, rhs: Quantity<V, U2>) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
            _p: PhantomData,
        }
    }
}

impl<T, V, U1, U2> Div<Quantity<V, U2>> for Quantity<T, U1>
where
    T: Copy,
    V: Copy,
    T: Div<V>,
    U1: Unit,
    U2: Unit,
{
    type Output = Quantity<<T as Div<V>>::Output, UnitDiv<U1, U2>>;

    fn div(self, rhs: Quantity<V, U2>) -> Self::Output {
        Self::Output {
            value: self.value / rhs.value,
            _p: PhantomData,
        }
    }
}

impl<T, US> Display for Quantity<T, US>
where
    T: Display,
    US: Unit,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "{:.*}{}", p, self.value, US::default())
        } else {
            write!(f, "{}{}", self.value, US::default())
        }
    }
}

#[cfg(all(test, feature = "time", feature = "length"))]
mod tests {
    use crate::length::*;
    use crate::quantity::errors::ConversionError;
    use crate::quantity::errors::ConversionError::NumCastFailed;
    use crate::scale::*;
    //use crate::temperature::DegreesCelsius;
    use crate::time::*;
    use crate::unit::helpers::{Alias, Div, Mul, Pow, Scale};
    use crate::{Quantity, base_unit};
    use alloc::format;
    use core::hash::Hash;
    use core::marker::PhantomData;

    #[test]
    fn test_construction() {
        use crate::length::Meters;
        assert_eq!(
            Meters::<i32>::new(42),
            Quantity::<i32, base_unit::Meters> {
                value: 42,
                _p: PhantomData
            }
            .alias()
        );
        assert_eq!(
            Meters::<f32>::new(42.5),
            Quantity::<f32, base_unit::Meters> {
                value: 42.5,
                _p: PhantomData
            }
        );
    }

    #[test]
    fn test_value() {
        use crate::length::Meters;
        assert_eq!(Meters::<i32>::new(42).value(), 42);
        assert_eq!(Meters::<f32>::new(42.5).value(), 42.5);
    }

    #[test]
    fn test_mul() {
        use crate::length::Meters;
        assert_eq!(
            Meters::<i32>::new(6) * Seconds::<i32>::new(3),
            Quantity::<i32, Mul<base_unit::Meters, base_unit::Seconds>> {
                value: 18,
                _p: PhantomData
            }
        );

        assert_eq!(
            (Quantity::<i32, base_unit::Meters>::new(6)
                * Quantity::<i32, base_unit::Meters>::new(3))
            .alias(),
            Quantity::<i32, Pow<base_unit::Meters, 2>> {
                value: 18,
                _p: PhantomData,
            }
        );
        assert_eq!(
            KiloMeters::<i32>::new(6) * Minutes::<i32>::new(3),
            Quantity::<
                i32,
                Mul<
                    Scale<base_unit::Meters, const { Rational::new(1000, 1) }>,
                    Alias<Scale<base_unit::Seconds, const { Rational::new(60, 1) }>, "min">,
                >,
            > {
                value: 18,
                _p: PhantomData,
            }
        );
    }

    #[test]
    fn test_div() {
        use crate::length::Meters;
        assert_eq!(
            Meters::<i32>::new(6) / Seconds::<i32>::new(3),
            Quantity::<i32, Div<base_unit::Meters, base_unit::Seconds>> {
                value: 2,
                _p: PhantomData,
            }
        );
        assert_eq!(
            Meters::<i32>::new(6) / Meters::<i32>::new(3),
            Quantity::<i32, base_unit::Unitless> {
                value: 2,
                _p: PhantomData,
            }
            .alias()
        );
        assert_eq!(
            KiloMeters::<i32>::new(6) / Minutes::<i32>::new(3),
            Quantity::<
                i32,
                Div<
                    Scale<base_unit::Meters, const { Rational::new(1000, 1) }>,
                    Alias<Scale<base_unit::Seconds, const { Rational::new(60, 1) }>, "min">,
                >,
            > {
                value: 2,
                _p: PhantomData,
            }
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            crate::length::Meters::<i32>::new(6) + crate::length::Meters::<i32>::new(3),
            Quantity::<i32, base_unit::Meters> {
                value: 9,
                _p: PhantomData,
            }
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            crate::length::Meters::<i32>::new(6) - crate::length::Meters::<i32>::new(3),
            Quantity::<i32, base_unit::Meters> {
                value: 3,
                _p: PhantomData,
            }
        );
    }

    #[test]
    fn test_convert() {
        let sec: Seconds<u64> = Minutes::<u64>::new(6).convert();
        assert_eq!(sec.value(), 360);
    }

    #[test]
    fn test_try_convert() {
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
        let sec8_res: Result<
            Quantity<u8, Scale<base_unit::Seconds, const { Rational::new(1000, 1) }>>,
            _,
        > = Seconds::<u8>::new(1).try_convert();
        assert_eq!(
            sec8_res,
            Err(ConversionError::DenominatorConversionError(NumCastFailed))
        );
    }

    #[test]
    fn test_format() {
        let meters = Meters::<f32>::new(10.1239);
        assert_eq!(format!("{}", meters), "10.1239m");
        assert_eq!(format!("{:.2}", meters), "10.12m");
        assert_eq!(format!("{:.1}", meters), "10.1m");
        assert_eq!(format!("{:.3}", meters), "10.124m");
    }

    #[test]
    fn test_hash() {
        use core::hash::Hasher;

        #[derive(Default)]
        struct RecordingHasher(alloc::vec::Vec<u8>);
        impl Hasher for RecordingHasher {
            fn finish(&self) -> u64 {
                self.0
                    .iter()
                    .fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64))
            }
            fn write(&mut self, bytes: &[u8]) {
                self.0.extend_from_slice(bytes);
            }
        }

        fn hash_of<H: Hash>(v: &H) -> u64 {
            let mut hasher = RecordingHasher::default();
            v.hash(&mut hasher);
            hasher.finish()
        }

        assert_eq!(hash_of(&Meters::<i32>::new(42)), hash_of(&42));
    }
}
