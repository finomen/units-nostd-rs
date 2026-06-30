use core::marker::{ConstParamTy, PhantomData};

#[derive(Default, Debug, Clone, Copy)]
pub struct NormalizedUnitType<U, const POW: i32, Tail = ()> {
    _p: PhantomData<(U, Tail)>,
}

trait NormalizedUnitTypeMarker {}

impl<U, const POW: i32, Tail> NormalizedUnitTypeMarker for NormalizedUnitType<U, POW, Tail> {}

pub trait NormalizedUnitAdd<U2, const POW2: i32> {
    type Result;
}

pub trait NormalizedUnitPow<const POW2: i32> {
    type Result;
}

#[derive(Debug, Clone, Copy, ConstParamTy)]
#[derive_const(PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstOrd {
    Less,
    Greater,
    Equal,
}
pub struct CompareResult<const ORDER: ConstOrd>;

pub const fn compare<A, B>() -> ConstOrd {
    let na = core::any::type_name::<A>().as_bytes();
    let nb = core::any::type_name::<B>().as_bytes();
    if na < nb {
        ConstOrd::Less
    } else if na > nb {
        ConstOrd::Greater
    } else {
        ConstOrd::Equal
    }
}

pub trait Compare {
    type Result;
}

impl<A, B> Compare for (A, B)
where
    CompareResult<const { compare::<A, B>() }>: Sized,
{
    type Result = CompareResult<const { compare::<A, B>() }>;
}

impl<U2, const POW2: i32> NormalizedUnitAdd<U2, POW2> for () {
    type Result = NormalizedUnitType<U2, POW2>;
}

pub trait NormalizedUnitAddImpl<U2, const POW2: i32, CR> {
    type Result;
}

impl<U1, const POW1: i32, Tail, U2, const POW2: i32>
    NormalizedUnitAddImpl<U2, POW2, CompareResult<{ ConstOrd::Less }>>
    for NormalizedUnitType<U1, POW1, Tail>
where
    Tail: NormalizedUnitAdd<U2, POW2>,
{
    type Result = NormalizedUnitType<U1, POW1, <Tail as NormalizedUnitAdd<U2, POW2>>::Result>;
}

impl<U1, const POW1: i32, Tail, U2, const POW2: i32>
    NormalizedUnitAddImpl<U2, POW2, CompareResult<{ ConstOrd::Greater }>>
    for NormalizedUnitType<U1, POW1, Tail>
where
    Tail: NormalizedUnitAdd<U1, POW1>,
{
    type Result = NormalizedUnitType<U2, POW2, <Tail as NormalizedUnitAdd<U1, POW1>>::Result>;
}

impl<U1, const POW1: i32, Tail, U2, const POW2: i32>
    NormalizedUnitAddImpl<U2, POW2, CompareResult<{ ConstOrd::Equal }>>
    for NormalizedUnitType<U1, POW1, Tail>
where
    DropZero<const { POW1 + POW2 == 0 }>: NormalizedUnitDropZero<U2, const { POW1 + POW2 }, Tail>,
{
    type Result = <DropZero<const { POW1 + POW2 == 0 }> as NormalizedUnitDropZero<
        U2,
        const { POW1 + POW2 },
        Tail,
    >>::Result;
}

// When two equal units' powers cancel to zero, drop the entry entirely so the
// normal form stays canonical (e.g. m/m must normalize to `()`, not `[(m,0)]`).
pub struct DropZero<const IS_ZERO: bool>;

pub trait NormalizedUnitDropZero<U, const POW: i32, Tail> {
    type Result;
}

impl<U, const POW: i32, Tail> NormalizedUnitDropZero<U, POW, Tail> for DropZero<true> {
    type Result = Tail;
}

impl<U, const POW: i32, Tail> NormalizedUnitDropZero<U, POW, Tail> for DropZero<false>
where
    NormalizedUnitType<U, POW, Tail>: Sized,
{
    type Result = NormalizedUnitType<U, POW, Tail>;
}

impl<U1, const POW1: i32, Tail, U2, const POW2: i32> NormalizedUnitAdd<U2, POW2>
    for NormalizedUnitType<U1, POW1, Tail>
where
    (U1, U2): Compare,
    NormalizedUnitType<U1, POW1, Tail>:
        NormalizedUnitAddImpl<U2, POW2, <(U1, U2) as Compare>::Result>,
{
    type Result = <NormalizedUnitType<U1, POW1, Tail> as NormalizedUnitAddImpl<
        U2,
        POW2,
        <(U1, U2) as Compare>::Result,
    >>::Result;
}

impl<const POW2: i32> NormalizedUnitPow<POW2> for () {
    type Result = ();
}

impl<U1, const POW1: i32, Tail, const POW2: i32> NormalizedUnitPow<POW2>
    for NormalizedUnitType<U1, POW1, Tail>
where
    Tail: NormalizedUnitPow<POW2>,
    NormalizedUnitType<U1, const { POW1 * POW2 }, <Tail as NormalizedUnitPow<POW2>>::Result>: Sized,
{
    type Result =
        NormalizedUnitType<U1, const { POW1 * POW2 }, <Tail as NormalizedUnitPow<POW2>>::Result>;
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

impl<U1, const POW1: i32, Tail1, L1, NL> NormalizedUnitMerge<NormalizedUnitType<U1, POW1, Tail1>>
    for L1
where
    L1: NormalizedUnitAdd<U1, POW1, Result = NL> + NormalizedUnitTypeMarker,
    NL: NormalizedUnitMerge<Tail1>,
{
    type Result = <NL as NormalizedUnitMerge<Tail1>>::Result;
}

#[cfg(test)]
mod tests {
    use crate::normalized_unit::{ConstOrd, compare};

    struct A;
    struct B;
    #[test]
    fn test_compare() {
        assert_eq!(compare::<A, B>(), ConstOrd::Less);
        assert_eq!(compare::<B, A>(), ConstOrd::Greater);
        assert_eq!(compare::<A, A>(), ConstOrd::Equal);
    }
}
