use core::marker::ConstParamTy;
use core::ops::{Add, Div, Mul, Sub};

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
