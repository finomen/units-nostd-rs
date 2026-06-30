use crate::base_unit::BaseUnit;
use core::marker::PhantomData;
use core::ops::Add;
use typenum::{B0, B1, Bit, Cmp, Equal, Greater, Integer, IsEqual, Less, Z0};

#[derive(Default, Debug, Clone, Copy)]
pub struct NormalizedUnitType<U, Pow, Tail = ()> {
    _p: PhantomData<(U, Pow, Tail)>,
}

trait NormalizedUnitTypeMarker {}

impl<U, Pow: Integer, Tail> NormalizedUnitTypeMarker for NormalizedUnitType<U, Pow, Tail> {}

pub trait NormalizedUnitAdd<U2, Pow: Integer> {
    type Result;
}

pub trait NormalizedUnitPow<Pow: Integer> {
    type Result;
}

impl<U2, Pow2: Integer> NormalizedUnitAdd<U2, Pow2> for () {
    type Result = NormalizedUnitType<U2, Pow2>;
}

pub trait NormalizedUnitAddImpl<U2, Pow2: Integer, CR> {
    type Result;
}

impl<U1, Pow1: Integer, Tail, U2, Pow2: Integer> NormalizedUnitAddImpl<U2, Pow2, Less>
    for NormalizedUnitType<U1, Pow1, Tail>
where
    Tail: NormalizedUnitAdd<U2, Pow2>,
{
    type Result = NormalizedUnitType<U1, Pow1, <Tail as NormalizedUnitAdd<U2, Pow2>>::Result>;
}

impl<U1, Pow1: Integer, Tail, U2, Pow2: Integer> NormalizedUnitAddImpl<U2, Pow2, Greater>
    for NormalizedUnitType<U1, Pow1, Tail>
where
    Tail: NormalizedUnitAdd<U1, Pow1>,
{
    type Result = NormalizedUnitType<U2, Pow2, <Tail as NormalizedUnitAdd<U1, Pow1>>::Result>;
}

impl<U1, Pow1: Integer, Tail, U2, Pow2: Integer, PowSum: Integer, IsZero: Bit>
    NormalizedUnitAddImpl<U2, Pow2, Equal> for NormalizedUnitType<U1, Pow1, Tail>
where
    Pow1: Add<Pow2, Output = PowSum>,
    PowSum: IsEqual<Z0, Output = IsZero>,
    DropZero<IsZero>: NormalizedUnitDropZero<U2, PowSum, Tail>,
{
    type Result = <DropZero<IsZero> as NormalizedUnitDropZero<U2, PowSum, Tail>>::Result;
}

// When two equal units' powers cancel to zero, drop the entry entirely so the
// normal form stays canonical (e.g. m/m must normalize to `()`, not `[(m,0)]`).
pub struct DropZero<IsZero: Bit> {
    _p: PhantomData<IsZero>,
}

pub trait NormalizedUnitDropZero<U, Pow: Integer, Tail> {
    type Result;
}

impl<U, Pow: Integer, Tail> NormalizedUnitDropZero<U, Pow, Tail> for DropZero<B1> {
    type Result = Tail;
}

impl<U, Pow: Integer, Tail> NormalizedUnitDropZero<U, Pow, Tail> for DropZero<B0>
where
    NormalizedUnitType<U, Pow, Tail>: Sized,
{
    type Result = NormalizedUnitType<U, Pow, Tail>;
}

impl<U1, Pow1: Integer, Tail, U2, Pow2: Integer, Cr> NormalizedUnitAdd<U2, Pow2>
    for NormalizedUnitType<U1, Pow1, Tail>
where
    U1: BaseUnit,
    U2: BaseUnit,
    U1::Id: Cmp<U2::Id, Output = Cr>,
    NormalizedUnitType<U1, Pow1, Tail>: NormalizedUnitAddImpl<U2, Pow2, Cr>,
{
    type Result =
        <NormalizedUnitType<U1, Pow1, Tail> as NormalizedUnitAddImpl<U2, Pow2, Cr>>::Result;
}

impl<Pow2: Integer> NormalizedUnitPow<Pow2> for () {
    type Result = ();
}

impl<U1, Pow1: Integer, Tail, Pow2: Integer, PowMul: Integer> NormalizedUnitPow<Pow2>
    for NormalizedUnitType<U1, Pow1, Tail>
where
    Tail: NormalizedUnitPow<Pow2>,
    Pow1: core::ops::Mul<Pow2, Output = PowMul>,
    NormalizedUnitType<U1, PowMul, <Tail as NormalizedUnitPow<Pow2>>::Result>: Sized,
{
    type Result = NormalizedUnitType<U1, PowMul, <Tail as NormalizedUnitPow<Pow2>>::Result>;
}

pub trait NormalizedUnitMerge<Other> {
    type Result;
}

impl NormalizedUnitMerge<()> for () {
    type Result = ();
}

impl<T: NormalizedUnitTypeMarker> NormalizedUnitMerge<T> for () {
    type Result = T;
}

impl<T: NormalizedUnitTypeMarker> NormalizedUnitMerge<()> for T {
    type Result = T;
}

impl<U1, Pow1: Integer, Tail1, L1, NL> NormalizedUnitMerge<NormalizedUnitType<U1, Pow1, Tail1>>
    for L1
where
    L1: NormalizedUnitAdd<U1, Pow1, Result = NL> + NormalizedUnitTypeMarker,
    NL: NormalizedUnitMerge<Tail1>,
{
    type Result = <NL as NormalizedUnitMerge<Tail1>>::Result;
}
