use crate::quantity::{UnitAdd, UnitDiv, UnitMul, UnitSub};
use core::marker::ConstParamTy;
use core::ops::{Add, Div, Mul, Sub};
use paste::paste;

macro_rules! si_compound_unit {
    ({ $( $base_unit:ident ),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, ConstParamTy)]
        #[derive_const(PartialEq, Eq)]
        #[doc(hidden)]
        /// Represents unit as product of powers of SI units.
        /// Constructed via the provided unit aliases, not directly.
        pub struct SiCompoundUnit {
            $(pub(crate)  $base_unit : i32),*
        }

        impl SiCompoundUnit {
            pub(crate) const fn pow(self, p: i32) -> Self {
                Self {
                    $($base_unit: self.$base_unit * p),*
                }
            }

            pub(crate) const fn zero() -> Self {
                Self {
                    $($base_unit: 0),*
                }
            }

            $(pub(crate) const fn $base_unit(mut self, v: i32) -> Self {
                self.$base_unit = v;
                self
            })*
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

        paste! {
            $(pub(crate) type [<$base_unit:camel>]<const P: i32> = SiCompoundUnitWrapper<{SiCompoundUnit::zero().$base_unit(P)}>;)*
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

impl<const U1: SiCompoundUnit, const U2: SiCompoundUnit> UnitMul<SiCompoundUnitWrapper<U2>>
    for SiCompoundUnitWrapper<U1>
where
    SiCompoundUnitWrapper<{ U1 * U2 }>: Sized,
{
    type Result = SiCompoundUnitWrapper<{ U1 * U2 }>;
}

impl<const U1: SiCompoundUnit, const U2: SiCompoundUnit> UnitDiv<SiCompoundUnitWrapper<U2>>
    for SiCompoundUnitWrapper<U1>
where
    SiCompoundUnitWrapper<{ U1 / U2 }>: Sized,
{
    type Result = SiCompoundUnitWrapper<{ U1 / U2 }>;
}

impl<const U: SiCompoundUnit> UnitAdd<SiCompoundUnitWrapper<U>> for SiCompoundUnitWrapper<U> {
    type Result = SiCompoundUnitWrapper<U>;
}

impl<const U: SiCompoundUnit> UnitSub<SiCompoundUnitWrapper<U>> for SiCompoundUnitWrapper<U> {
    type Result = SiCompoundUnitWrapper<U>;
}
