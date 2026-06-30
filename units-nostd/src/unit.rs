pub(crate) mod helpers;
pub mod ops;

use crate::Rational;
use core::fmt::Display;
use core::marker::PhantomData;

pub trait Tag<T> {
    const VALUE: T;
}

pub trait UnitTagMarker {}

pub struct UnitTag<U> {
    _p: PhantomData<U>,
}

pub struct ScaleTag<const S: Rational>;

impl<T> UnitTagMarker for UnitTag<T> {}

impl<const S: Rational> Tag<Rational> for ScaleTag<{ S }> {
    const VALUE: Rational = S;
}

pub trait UnitTags {
    type Unit;
    type SymbolScale;
}

pub const trait Unit:
    ops::Mul + ops::Div + Default + Copy + ops::Pow + ops::Scale + Display
{
    type Normalized;
    const SYMBOL_SCALE: Rational;
}

pub(crate) const trait NamedUnit {}

impl<T> UnitTags for T
where
    T: Unit,
    ScaleTag<{ T::SYMBOL_SCALE }>: Sized,
{
    type Unit = UnitTag<T::Normalized>;
    type SymbolScale = ScaleTag<{ T::SYMBOL_SCALE }>;
}

pub trait Symbol {
    const SYMBOL: &'static str;
}
