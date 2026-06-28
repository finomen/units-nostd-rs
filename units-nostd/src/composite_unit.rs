use crate::base_unit::Unitless;
use crate::normalized_unit::{NormalizedUnitMerge, NormalizedUnitPow};
use crate::scale::Rational;
use crate::unit::{NamedUnit, Unit};
use core::fmt::{Display, Formatter};
use core::marker::PhantomData;

trait DisplayOps {
    const PARENTHESES_FOR_MUL: bool;
    const PARENTHESES_FOR_DIV: bool;
    const PARENTHESES_FOR_POW: bool;
}

impl<T: NamedUnit> DisplayOps for T {
    const PARENTHESES_FOR_MUL: bool = false;
    const PARENTHESES_FOR_DIV: bool = false;
    const PARENTHESES_FOR_POW: bool = false;
}
impl<A, B> DisplayOps for UnitMul<A, B> {
    const PARENTHESES_FOR_MUL: bool = false;
    const PARENTHESES_FOR_DIV: bool = true;
    const PARENTHESES_FOR_POW: bool = true;
}
impl<A, B> DisplayOps for UnitDiv<A, B> {
    const PARENTHESES_FOR_MUL: bool = true;
    const PARENTHESES_FOR_DIV: bool = true;
    const PARENTHESES_FOR_POW: bool = true;
}
impl<A, const POW: i32> DisplayOps for UnitPow<A, POW> {
    const PARENTHESES_FOR_MUL: bool = false;
    const PARENTHESES_FOR_DIV: bool = false;
    const PARENTHESES_FOR_POW: bool = true;
}
impl<A, const S: Rational> DisplayOps for UnitScale<A, S> {
    const PARENTHESES_FOR_MUL: bool = false;
    const PARENTHESES_FOR_DIV: bool = false;
    const PARENTHESES_FOR_POW: bool = false;
}

impl DisplayOps for Unitless {
    const PARENTHESES_FOR_MUL: bool = false;
    const PARENTHESES_FOR_DIV: bool = false;
    const PARENTHESES_FOR_POW: bool = false;
}

#[derive(Debug, Clone, Copy)]
pub struct UnitMul<A, B> {
    _p: PhantomData<(A, B)>,
}

const impl<A, B> Default for UnitMul<A, B>
where
    A: [const] Default,
    B: [const] Default,
{
    fn default() -> Self {
        Self { _p: PhantomData }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnitDiv<A, B> {
    _p: PhantomData<(A, B)>,
}

const impl<A, B> Default for UnitDiv<A, B>
where
    A: [const] Default,
    B: [const] Default,
{
    fn default() -> Self {
        Self { _p: PhantomData }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnitPow<A, const POW: i32> {
    _p: PhantomData<A>,
}

const impl<A, const POW: i32> Default for UnitPow<A, POW>
where
    A: [const] Default,
{
    fn default() -> Self {
        Self { _p: PhantomData }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnitScale<A, const S: Rational> {
    _p: PhantomData<A>,
}

const impl<A, const S: Rational> Default for UnitScale<A, S>
where
    A: [const] Default,
{
    fn default() -> Self {
        Self { _p: PhantomData }
    }
}

const impl<A, B, NA, NB> Unit for UnitMul<A, B>
where
    A: [const] Unit<Normalized = NA> + Copy + DisplayOps,
    B: [const] Unit<Normalized = NB> + Copy + DisplayOps,
    NA: NormalizedUnitMerge<NB>,
{
    type Normalized = <NA as NormalizedUnitMerge<NB>>::Result;
    const SYMBOL_SCALE: Rational = A::SYMBOL_SCALE * B::SYMBOL_SCALE;
}

impl<A, B> Display for UnitMul<A, B>
where
    A: Unit + Copy + DisplayOps,
    B: Unit + Copy + DisplayOps,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match (A::PARENTHESES_FOR_MUL, B::PARENTHESES_FOR_MUL) {
            (false, false) => write!(f, "{}·{}", A::default(), B::default()),
            (true, false) => write!(f, "({})·{}", A::default(), B::default()),
            (false, true) => write!(f, "{}·({})", A::default(), B::default()),
            (true, true) => write!(f, "({})·({})", A::default(), B::default()),
        }
    }
}

const impl<A, B, NA, NB, NBN> Unit for UnitDiv<A, B>
where
    A: [const] Unit<Normalized = NA> + Copy + DisplayOps,
    B: [const] Unit<Normalized = NB> + Copy + DisplayOps,
    NB: NormalizedUnitPow<-1, Result = NBN>,
    NA: NormalizedUnitMerge<NBN>,
{
    type Normalized = <NA as NormalizedUnitMerge<NBN>>::Result;
    const SYMBOL_SCALE: Rational = A::SYMBOL_SCALE / B::SYMBOL_SCALE;
}

impl<A, B> Display for UnitDiv<A, B>
where
    A: Unit + Copy + DisplayOps,
    B: Unit + Copy + DisplayOps,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match (A::PARENTHESES_FOR_DIV, B::PARENTHESES_FOR_DIV) {
            (false, false) => write!(f, "{}/{}", A::default(), B::default()),
            (true, false) => write!(f, "({})/{}", A::default(), B::default()),
            (false, true) => write!(f, "{}/({})", A::default(), B::default()),
            (true, true) => write!(f, "({})/({})", A::default(), B::default()),
        }
    }
}

const impl<A, const POW: i32, NA> Unit for UnitPow<A, POW>
where
    A: [const] Unit<Normalized = NA> + Copy + DisplayOps,
    NA: NormalizedUnitPow<POW>,
{
    type Normalized = <NA as NormalizedUnitPow<POW>>::Result;
    const SYMBOL_SCALE: Rational = A::SYMBOL_SCALE.pow(POW);
}

impl<A, const POW: i32> Display for UnitPow<A, POW>
where
    A: Unit + Copy + DisplayOps,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        const SUPERSCRIPTS: [char; 11] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹', '⁻'];

        fn push_char(buf: &mut [u8; 48], pos: usize, c: char) -> usize {
            let mut tmp = [0u8; 4];
            let encoded = c.encode_utf8(&mut tmp).as_bytes();
            let start = pos - encoded.len();
            buf[start..pos].copy_from_slice(encoded);
            start
        }

        pub(crate) fn pow_str(mut p: i32, buf: &mut [u8; 48]) -> &str {
            let mut pos = buf.len();
            let sign = p < 0;
            if p < 0 {
                p = -p;
            }
            while p > 0 {
                let digit = (p % 10) as usize;
                pos = push_char(buf, pos, SUPERSCRIPTS[digit]);
                p /= 10;
            }
            if sign {
                pos = push_char(buf, pos, SUPERSCRIPTS[10]);
            }
            unsafe { core::str::from_utf8_unchecked(&buf[pos..]) }
        }
        let mut buf: [u8; 48] = [0; 48];
        let pstr = pow_str(POW, &mut buf);

        if A::PARENTHESES_FOR_POW {
            write!(f, "({}){}", A::default(), pstr)
        } else {
            write!(f, "{}{}", A::default(), pstr)
        }
    }
}

const impl<A, const S: Rational> Unit for UnitScale<A, S>
where
    A: [const] Unit + Copy,
{
    type Normalized = A::Normalized;
    const SYMBOL_SCALE: Rational = A::SYMBOL_SCALE * S;
}

impl<A, const S: Rational> Display for UnitScale<A, S>
where
    A: Unit + Copy,
{
    default fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{}]({})", S, A::default())
    }
}

impl<A, const S: Rational> Display for UnitScale<A, S>
where
    A: Unit + Copy + NamedUnit,
    crate::prefixes::Scaled<S>: crate::prefixes::NamedScale,
{
    default fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}{}",
            <crate::prefixes::Scaled::<S> as crate::prefixes::NamedScale>::PREFIX,
            A::default()
        )
    }
}

impl<A, const P: i32, const S: Rational> Display for UnitScale<UnitPow<A, P>, S>
where
    A: Unit + Copy + NamedUnit,
    UnitPow<A, P>: Unit,
    crate::prefixes::Scaled<const { S.nth_root(P) }>: crate::prefixes::NamedScale,
{
    default fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", <crate::prefixes::Scaled::<const {S.nth_root(P)}> as crate::prefixes::NamedScale>::PREFIX, UnitPow::<A, P>::default())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnitAlias<A, const SYMBOL: &'static str> {
    _p: PhantomData<A>,
}

impl<A, const SYMBOL: &'static str> NamedUnit for UnitAlias<A, SYMBOL> {}

const impl<A, const SYMBOL: &'static str> Unit for UnitAlias<A, SYMBOL>
where
    A: [const] Unit + Copy,
{
    type Normalized = A::Normalized;
    const SYMBOL_SCALE: Rational = A::SYMBOL_SCALE;
}

impl<A, const SYMBOL: &'static str> Display for UnitAlias<A, SYMBOL> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(SYMBOL)
    }
}

const impl<A, const SYMBOL: &'static str> Default for UnitAlias<A, SYMBOL>
where
    A: [const] Unit + Copy + [const] Default,
{
    fn default() -> Self {
        Self { _p: PhantomData }
    }
}

#[cfg(test)]
mod tests {
    use crate::Rational;
    use crate::base_unit::*;
    use crate::unit::helpers::{Alias, Div, Mul, Pow, Scale};
    use alloc::format;

    #[test]
    fn test_mul() {
        assert_eq!(format!("{}", Mul::<Meters, Seconds>::default()), "m·s");
        assert_eq!(
            format!("{}", Mul::<Mul::<Meters, Seconds>, Seconds>::default()),
            "m·s·s"
        );
        assert_eq!(
            format!("{}", Mul::<Div::<Meters, Seconds>, Seconds>::default()),
            "(m/s)·s"
        );
        assert_eq!(
            format!("{}", Mul::<Seconds, Div::<Meters, Seconds>>::default()),
            "s·(m/s)"
        );
        assert_eq!(
            format!(
                "{}",
                Mul::<Div::<Meters, Seconds>, Div::<Meters, Seconds>>::default()
            ),
            "(m/s)·(m/s)"
        );
    }

    #[test]
    fn test_div() {
        assert_eq!(format!("{}", Div::<Meters, Seconds>::default()), "m/s");
        assert_eq!(
            format!("{}", Div::<Div::<Meters, Seconds>, Seconds>::default()),
            "(m/s)/s"
        );
        assert_eq!(
            format!("{}", Div::<Meters, Mul::<Seconds, Seconds>>::default()),
            "m/(s·s)"
        );
        assert_eq!(
            format!(
                "{}",
                Div::<Mul::<Meters, Meters>, Mul::<Seconds, Seconds>>::default()
            ),
            "(m·m)/(s·s)"
        );
    }
    #[test]
    fn test_pow() {
        assert_eq!(format!("{}", Pow::<Seconds, -1>::default()), "s⁻¹");
        assert_eq!(format!("{}", Pow::<Seconds, -15>::default()), "s⁻¹⁵");
        assert_eq!(
            format!(
                "{}",
                Pow::<Scale::<Meters, const { Rational::new(1000, 1) }>, 2>::default()
            ),
            "km²"
        );
        assert_eq!(
            format!("{}", Pow::<Div::<Meters, Seconds>, 2>::default()),
            "(m/s)²"
        );
    }
    #[test]
    fn test_scale() {
        assert_eq!(
            format!(
                "{}",
                Scale::<Meters, const { Rational::new(1000, 1) }>::default()
            ),
            "km"
        );
        assert_eq!(
            format!(
                "{}",
                Scale::<Meters, const { Rational::new(70, 3) }>::default()
            ),
            "[70/3](m)"
        );
        assert_eq!(
            format!(
                "{}",
                Scale::<Pow::<Meters, 2>, const { Rational::new(1000000, 1) }>::default()
            ),
            "km²"
        );
    }
    #[test]
    fn test_alias() {
        assert_eq!(format!("{}", Alias::<Meters, "dummy">::default()), "dummy");
    }
}
