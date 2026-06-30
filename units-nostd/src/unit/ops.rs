use crate::Rational;
use crate::composite_unit::{UnitAlias, UnitDiv, UnitMul, UnitPow, UnitScale};
use crate::unit::{Symbol, Unit};
use typenum::Integer;

pub const trait Mul {
    type Mul<U: crate::unit::Unit>: crate::unit::Unit + [const] Default
    where
        UnitMul<Self, U>: Unit;
}
pub const trait Div {
    type Div<U: crate::unit::Unit>: crate::unit::Unit + [const] Default
    where
        UnitDiv<Self, U>: Unit;
}

pub const trait Pow {
    type Pow<Pow: Integer>: crate::unit::Unit + [const] Default
    where
        UnitPow<Self, Pow>: Unit;
}

pub const trait Scale {
    type Scale<const S: crate::Rational>: crate::unit::Unit + [const] Default
    where
        UnitScale<Self, S>: Unit;
}

pub const trait Alias {
    type Alias<S: Symbol>: crate::unit::Unit + [const] Default
    where
        UnitAlias<Self, S>: Unit;
}

impl<U1> Mul for U1
where
    U1: Unit,
{
    type Mul<U2: Unit>
        = UnitMul<U1, U2>
    where
        UnitMul<U1, U2>: Unit;
}

impl<U1> Div for U1
where
    U1: Unit,
{
    type Div<U2: Unit>
        = UnitDiv<U1, U2>
    where
        UnitDiv<U1, U2>: Unit;
}

impl<U1> Pow for U1
where
    U1: Unit,
{
    type Pow<Pow: Integer>
        = UnitPow<U1, Pow>
    where
        UnitPow<Self, Pow>: Unit;
}

impl<U1> Scale for U1
where
    U1: Unit,
{
    type Scale<const S: Rational>
        = UnitScale<U1, S>
    where
        UnitScale<Self, S>: Unit;
}

impl<U1> Alias for U1
where
    U1: Unit,
{
    type Alias<S: Symbol>
        = UnitAlias<U1, S>
    where
        UnitAlias<Self, S>: Unit;
}
